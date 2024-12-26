use cw_storage_plus::Map;

pub const ITEM_COUNTS: Map<&'static str, u32> = Map::new("item_count");
