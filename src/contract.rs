use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{ItemCountResp, ItemsResp, QueryMsg};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ItemCount => to_json_binary(&ItemCountResp { count: 0 }),
        QueryMsg::Itmes => to_json_binary(&ItemsResp { items: [] }),
    }
}
