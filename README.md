# cosm-vending-machine

A small contract vending machine which allows anyone to take an item but only 
allows the creator/owner of the contract to refill items.

`items`
--
- `snacks`
- `chocolate`
- `water`
- `chips`

`init contract`
--
You must initalize the contract with the:
- `owner`: the owners address 
- `snacks_count`: the amount of snacks available
- `chocolate_count`: the amount of chocolate available
- `water_count`: the amount of water available
- `chips_count`: the amount of chips available

```json 
{"snacks_count": 100,"chocolate_count": 100,"water_count": 100,"chips_count": 100}
```

`queries`
--
### `item_count`
retruns the available count for an item

- `params`: an `Item`
- `output`: an `unsigned int`

### `items`
returns the list of items

- `params`: None
- `output`: an list of `Item`

`functions`
--
### `get_item`
removes one count from the given item

- `params`: an `Item`
- `output`: None

### `refill`
refill item by given item, count pair. Note the count it increased by the given
amount.

This function can only be executed by the owner of the contract.

- `params`: an item(`Item`) and a count(`unsigned int`)
- `output`: None


Deploy and Run
--

build wasm file
```
docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:0.16.0
```

deploy contract
```
osmosisd tx wasm store artifacts/cosm_vending_machine.wasm --from wallet --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -y --output json

{"height":"0","txhash":"FD0346B317C3E007EC7569381634F89A3BD9326358D6A751FE3EBC19C9C7CC23","codespace":"","code":0,"data":"","raw_log":"","logs":[],"info":"","gas_wanted":"0","gas_used":"0","tx":null,"timestamp":"","events":[]}
```

use `txhash` to get the `code_id`

init contract
```
CODE_ID=11915
INIT='{"snacks_count": 100,"chocolate_count": 100,"water_count": 100,"chips_count": 100}'

osmosisd tx wasm instantiate $CODE_ID "$INIT" \
    --from wallet --label "init cosm_vending_machine" --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y --no-admin

gas estimate: 245395
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: 55B109E5CED1075DF5EA8ABCD3B1D6A02C73AED5519761B5F97321B1C5A6DEAA
```

get contract addr
```
CONTRACT_ADDR=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')
```

get_items from contract
```
CODE_ID=11915
QUERY='"itmes"'

osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json

{"data":{"items":["snacks","chocolate","water","chips"]}}
```

item_count from contract
```
CODE_ID=11915
QUERY='{"item_count": {"item": "chocolate"}}'

osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json

{"data":{"item":"chocolate","count":100}}
```

exec item from contract
```
CODE_ID=11915
ITEM='{"get_item": {"item": "chocolate"}}'

osmosisd tx wasm execute $CONTRACT_ADDR "$ITEM" --from wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y --chain-id osmo-test-5

gas estimate: 176053
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: FC977DA4DEE4338BB93DF010DF7D46067E41497D4D02028D5E6F9EBC67297383
```

exec refill from contract
```
CODE_ID=11915
ITEM='{"refill": {"item": "chocolate", "count": 100}}'

osmosisd tx wasm execute $CONTRACT_ADDR "$ITEM" --from wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y --chain-id osmo-test-5

gas estimate: 178371
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: 46DC36262C26A0BA9ACA67BF1A50847811CBD8EDC90A2B5E1D6A65EB159366BE
```
