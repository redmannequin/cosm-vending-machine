use cosmwasm_schema::cw_serde;
use cosmwasm_std::CustomQuery;

#[cw_serde]
pub enum QueryMsg {
    ItemCount,
    Itmes,
}

impl CustomQuery for QueryMsg {}

#[cw_serde]
pub struct ItemCountResp {
    pub count: u32,
}

#[cw_serde]
pub struct ItemsResp {
    pub items: [&'static str; 0],
}
