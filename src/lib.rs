mod contract;
pub mod msg;
mod state;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use msg::{InstantiateMsg, QueryMsg};

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
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{
        execute, instantiate,
        msg::{InstantiateMsg, Item, ItemCountResp, ItemsResp, QueryMsg},
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
                creator,
                &InstantiateMsg {
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
                creator,
                &InstantiateMsg {
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
}
