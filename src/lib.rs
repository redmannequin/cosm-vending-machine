mod contract;
pub mod msg;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use msg::QueryMsg;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
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
        msg::{ItemCountResp, ItemsResp, QueryMsg},
        query,
    };
    use cosmwasm_std::{testing::mock_dependencies, Empty};
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn item_count_query() {
        let deps = mock_dependencies();
        let creator = deps.api.addr_make("creator");

        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(code_id, creator, &Empty {}, &[], "Contract", None)
            .expect("failed to setup contract");

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::ItemCount)
            .expect("failed to query contract");

        assert_eq!(resp, ItemCountResp { count: 0 })
    }

    #[test]
    fn items_query() {
        let deps = mock_dependencies();
        let creator = deps.api.addr_make("creator");

        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(code_id, creator, &Empty {}, &[], "Contract", None)
            .expect("failed to setup contract");

        let resp: ItemCountResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Itmes)
            .expect("failed to query contract");

        assert_eq!(resp, ItemsResp { items: [] })
    }
}
