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