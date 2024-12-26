mod contract;
pub mod msg;
mod state;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    contract::execute(deps, env, info, msg)
}

#[cfg(test)]
mod tests {
    use crate::{
        execute, instantiate,
        msg::{ExecuteMsg, InstantiateMsg, Item, ItemCountResp, ItemsResp, QueryMsg},
        query,
    };
    use cosmwasm_std::testing::mock_dependencies;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn item_count_query() {
        let deps = mock_dependencies();
        let creator = deps.api.addr_make("creator");

        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                creator.clone(),
                &InstantiateMsg {
                    owner: creator.clone(),
                    snacks_count: 1,
                    chocolate_count: 2,
                    water_count: 3,
                    chips_count: 4,
                },
                &[],
                "Contract",
                None,
            )
            .expect("failed to setup contract");

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ItemCount { item: Item::Snacks })
            .expect("failed to query contract");

        assert_eq!(
            resp,
            ItemCountResp {
                item: Item::Snacks,
                count: 1
            }
        );

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::ItemCount { item: Item::Chips })
            .expect("failed to query contract");

        assert_eq!(
            resp,
            ItemCountResp {
                item: Item::Chips,
                count: 4
            }
        )
    }

    #[test]
    fn items_query() {
        let deps = mock_dependencies();
        let creator = deps.api.addr_make("creator");

        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                creator.clone(),
                &InstantiateMsg {
                    owner: creator.clone(),
                    snacks_count: 1,
                    chocolate_count: 2,
                    water_count: 3,
                    chips_count: 4,
                },
                &[],
                "Contract",
                None,
            )
            .expect("failed to setup contract");

        let resp: ItemsResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Itmes)
            .expect("failed to query contract");

        assert_eq!(
            resp,
            ItemsResp {
                items: [Item::Snacks, Item::Chocolate, Item::Water, Item::Chips]
            }
        )
    }

    #[test]
    fn get_item_exec() {
        let deps = mock_dependencies();
        let creator = deps.api.addr_make("creator");
        let other = deps.api.addr_make("other");

        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                creator.clone(),
                &InstantiateMsg {
                    owner: creator.clone(),
                    snacks_count: 1,
                    chocolate_count: 2,
                    water_count: 3,
                    chips_count: 4,
                },
                &[],
                "Contract",
                None,
            )
            .expect("failed to setup contract");

        app.execute_contract(
            other,
            addr.clone(),
            &ExecuteMsg::GetItem { item: Item::Snacks },
            &[],
        )
        .expect("failed to get_item(snacks) for other");

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ItemCount { item: Item::Snacks })
            .expect("failed to query contract");

        assert_eq!(
            resp,
            ItemCountResp {
                item: Item::Snacks,
                count: 0
            }
        );

        app.execute_contract(
            creator,
            addr.clone(),
            &ExecuteMsg::GetItem { item: Item::Snacks },
            &[],
        )
        .expect_err("removing from an empty item should fail");
    }

    #[test]
    fn refill_exec() {
        let deps = mock_dependencies();
        let creator = deps.api.addr_make("creator");
        let other = deps.api.addr_make("other");
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                creator.clone(),
                &InstantiateMsg {
                    owner: creator.clone(),
                    snacks_count: 0,
                    chocolate_count: 0,
                    water_count: 0,
                    chips_count: 0,
                },
                &[],
                "Contract",
                None,
            )
            .expect("failed to setup contract");

        app.execute_contract(
            other,
            addr.clone(),
            &ExecuteMsg::Refill {
                item: Item::Chips,
                count: 5,
            },
            &[],
        )
        .expect_err("No addr other than the creator should be allowed to refill");

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ItemCount { item: Item::Chips })
            .expect("failed to query contract");

        assert_eq!(
            resp,
            ItemCountResp {
                item: Item::Chips,
                count: 0
            }
        );

        app.execute_contract(
            creator,
            addr.clone(),
            &ExecuteMsg::Refill {
                item: Item::Chips,
                count: 5,
            },
            &[],
        )
        .expect("failed to refill item");

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ItemCount { item: Item::Chips })
            .expect("failed to query contract");

        assert_eq!(
            resp,
            ItemCountResp {
                item: Item::Chips,
                count: 5
            }
        );
    }
}
