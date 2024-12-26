use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, Item, ItemCountResp, ItemsResp, QueryMsg},
    state::ITEM_COUNTS,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ITEM_COUNTS.save(deps.storage, Item::Chips.as_str(), &msg.chips_count)?;
    ITEM_COUNTS.save(deps.storage, Item::Chocolate.as_str(), &msg.chocolate_count)?;
    ITEM_COUNTS.save(deps.storage, Item::Snacks.as_str(), &msg.snacks_count)?;
    ITEM_COUNTS.save(deps.storage, Item::Water.as_str(), &msg.water_count)?;
    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ItemCount { item } => to_json_binary(&ItemCountResp {
            item,
            count: ITEM_COUNTS.load(deps.storage, item.as_str())?,
        }),
        QueryMsg::Itmes => to_json_binary(&ItemsResp {
            items: [Item::Snacks, Item::Chocolate, Item::Water, Item::Chips],
        }),
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::GetItem { item } => {
            let count = ITEM_COUNTS.load(deps.storage, item.as_str())?;
            if count == 0 {
                Err(StdError::generic_err("Empty"))
            } else {
                ITEM_COUNTS.save(deps.storage, item.as_str(), &(count - 1))?;
                Ok(Response::new())
            }
        }
        ExecuteMsg::Refill { item, count } => {
            let curr_count = ITEM_COUNTS.load(deps.storage, item.as_str())?;
            ITEM_COUNTS.save(deps.storage, item.as_str(), &(curr_count + count))?;
            Ok(Response::new())
        }
    }
}
