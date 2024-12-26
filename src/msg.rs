use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CustomQuery};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub snacks_count: u32,
    pub chocolate_count: u32,
    pub water_count: u32,
    pub chips_count: u32,
}

#[cw_serde]
pub enum QueryMsg {
    ItemCount { item: Item },
    Itmes,
}

impl CustomQuery for QueryMsg {}

#[cw_serde]
pub struct ItemCountResp {
    pub item: Item,
    pub count: u32,
}

#[cw_serde]
pub struct ItemsResp {
    pub items: [Item; 4],
}

#[cw_serde]
pub enum ExecuteMsg {
    GetItem { item: Item },
    Refill { item: Item, count: u32 },
}

#[cw_serde]
#[derive(Copy)]
pub enum Item {
    Snacks,
    Chocolate,
    Water,
    Chips,
}

impl Item {
    pub const fn as_str(self) -> &'static str {
        match self {
            Item::Snacks => "snacks",
            Item::Chocolate => "shocolate",
            Item::Water => "sater",
            Item::Chips => "ships",
        }
    }
}
