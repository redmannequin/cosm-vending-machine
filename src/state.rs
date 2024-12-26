use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const ITEM_COUNTS: Map<&'static str, u32> = Map::new("item_count");
