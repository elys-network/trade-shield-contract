# README

This README provides documentation for the three main functions in the provided script, which interacts with a CosmWasm contract on a blockchain network.

## Functions

### 1. createSpotOrder( order_price, order_type, amount_send, denom_send, order_target_denom)

This function allows you to create a new spot order by sending a transaction to the CosmWasm contract.

#### Parameters

- `order_price` ({`base_denom`:String, `quote_denom`:String, `rate` :String} or null): Price relates two assets exchange rate that the user should define, can only be null if the order type is "market_type"
- `order_type` (String): The type of the order (e.g., "stop_loss", "limit_sell", "limit_buy", "market_buy").
- `amount_send` (String): The amount of cryptocurrency to send in the order.
- `denom_send` (String): The denomination of the cryptocurrency to send.
- `order_target_denom` (String) : the asset that the user want to convert their asset into

#### Usage

```javascript
createSpotOrder(
  {"base_denom", "quote_denom", "rate"},
  "order_type",
  "amount_to_send_here",
  "denom_to_send_here"
  "your_target_denom"
);

createSpotOrder(
  {"base_denom", "quote_denom", "rate"},
  "order_type",
  "amount_to_send_here",
  "denom_to_send_here"
  "your_target_denom"
);
```

#### Example

```js
createSpotOrder(
  { base_denom: "BTC", quote_denom: "ETH", rate: "0.035" },
  "limit_buy",
  "2.5",
  "ETH",
  "BTC"
);

createSpotOrder(
  { base_denom: "BTC", quote_denom: "ETH", rate: "0.035" },
  "limit_buy",
  "2.5",
  "ETH",
  "BTC"
);
```

### 2. cancelSpotOrder(order_id)

This function allows you to cancel an existing order by sending a transaction to the CosmWasm contract.

#### Parameters

- `order_id` (String): The unique identifier for the order you want to cancel.

#### Usage

```javascript
cancelSpotOrder("your_order_id_here");
```

#### Example

```js
cancelSpotOrder("8");
```

### 3. cancelSpotOrders(order_ids, order_type, owner_address)

This function retrieves information about multiple order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `order_ids` ([u64] or null): list of order ids that should be canceled
- `order_type` (OrderType or null): select the order type that should be canceled
- `owner_address` (String): select the owner of the order that should be canceled

#### Usage

```javascript
cancelSpotOrders("order_ids", "order_type", "order_owner");
```

#### Example

```js
cancelSpotOrders(
  [5, 4, 6],
  "limit_sell",
  "elys1x5fehwug2vtkyn4vpunwkfn9zxkpxl8jg0lwuu"
);
```

### 4. getSpotOrder(order_id)

This function retrieves information about a specific order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `order_id` (String): The unique identifier for the order you want to retrieve.

#### Usage

```javascript
getSpotOrder("your_order_id_here");
```

#### Example

```js
getSpotOrder("1");
```

### 5. getSpotOrders(pagination, order_type, order_owner, order_status)

This function retrieves information about multiple order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `pagination` {PageRequest} :
- `order_type` (OrderType or null): select the order type that should be querried
- `order_owner` (String or null): select the owner of the order that should be querried
- `order_status` (String or null) : select the order staus that should be querried (Pending,Executed,Canceled)

#### Usage

```javascript
getSpotOrders({"count_total", "limit", "reverse", "key"}, "order_type", "order_owner", "status")
```

####

```js
getSpotOrders(
  { count_total: true, limit: 10, reverse: false, key: null },
  "stop_loss",
  "elys12tzylat4udvjj56uuhu3vj2n4vgp7cf9fwna9w",
  null
);
```

### 6. createMarginOrder(position, collateral, leverage, trading_asset, take_profit_price, order_type, trigger_price)

This function allows you to create a margin order by sending a transaction to the CosmWasm contract.

#### Parameters

- `collateral` (Coin {demom: String , amount : String} or null): The amount of collateral for the margin order. Can only be null if it's not a LimitOpen or MarketOpen type.
- `position` (String or null): The type of position for the margin order (e.g., "long", "short"). Can be null if it's not a LimitOpen or MarketOpen type
- `leverage` (String or null): The leverage for the margin order.Can be null if it's not a LimitOpen or MarketOpen type
- `trading_asset` (String or null): The asset to borrow for the margin order. Can be null if it's not a LimitOpen or MarketOpen type
- `take_profit_price` (String or null): The price at which the order will take profit. Can be null if it's not a LimitOpen or MarketOpen type
- `order_type` (String): The type of the order (e.g., "stop_loss", "limit_sell", "limit_buy").
- `trigger_price` ({`base_denom`:String, `quote_denom`:String, `rate` :String} or null): Price relates two assets exchange rate that the user should define, can only be null if the order type is "market_type"
- `position_id` (u64 or null) Can be null if it's not a LimitClose, MarketClose or StopLoss type

#### Usage

```javascript
createMarginOrder(
  "position_type",
  "collateral",
  "leverage_value",
  "trading_asset",
  "take_profit_price",
  "order_type",
  "trigger_price"
  "position_id"
);
```

#### Example

```javascript
createMarginOrder(
  "short",
  { denom: "uusdc", amount: "2002" },
  "4.3",
  "ueth",
  "2.2"
  "limit_buy",
  { base_denom: "ueth", quote_denom: "uusdc", rate: "2076.5" }
  null
);
```

### 7. cancelMarginOrder(order_id)

This function allows you to cancel a margin order by sending a transaction to the CosmWasm contract.

#### Parameters

- `order_id` (String): The unique identifier for the order you want to cancel.

#### Usage

```javascript
cancelMarginOrder("your_order_id_here");
```

#### Example

```js
cancelMarginOrder("1");
```

### 8. getMarginOrder(id)

This function retrieves information about a specific margin order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `order_id` (String): The unique identifier for the order you want to retrieve.

#### Usage

```javascript
getMarginOrder("your_order_id_here");
```

#### Example

```js
getMarginOrder("2");
```

### 9. getMarginPosition(address,id)

This function retrieves information about a specific margin order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `address` (String): The address associated with the margin order.
- `order_id` (String): The unique identifier for the order you want to retrieve.

#### Usage

```javascript
getMarginPosition("your_address", "your_order_id_here");
```

#### Example

```js
getMarginPosition("elys1x5fehwug2vtkyn4vpunwkfn9zxkpxl8jg0lwuu", "255");
```

### 10. getMarginPositions(pagination)

This function retrieves multiple margin orders by querying a CosmWasm contract on the blockchain.

#### Parameters

- `pagination` {PageRequest} : Parameters for pagination.

#### Usage

```javascript
getMarginPositions("pagination");
```

#### Example

```js
getMarginPositions({ count_total: true, limit: 10, reverse: false, key: null });
```

### 11. SwapEstimationByDenom(amount, denom_in, denom_out, user_address)

This function retrieves an estimation of the value obtained by swapping one asset for another.

#### Parameters

- `amount` {Coin} : the amount of the value that you want to send or recive.
- `denom_in` (String) : The asset that you will send.
- `denom_out` (String) : The asset that you will recive.
- `user_address` (String or null): user_address to calculate the discount that the user have access

#### Usage

```js
 SwapEstimationByDenom({"amount", "denom"}, "denom_in", "denom_out", "user_address")
```

#### Example

```js
SwapEstimationByDenom({
  amount: { amount: 200, denom: "usdc" },
  denom_in: "usdc",
  denom_out: "atom",
  user_address: "elys12tzylat4udvjj56uuhu3vj2n4vgp7cf9fwna9w",
});
```

### 12. getMarginOrders(pagination, order_type, order_owner, order_status)

This function retrieves information about multiple order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `pagination` {PageRequest} :
- `order_type` (OrderType or null): select the order type that should be querried
- `order_owner` (String or null): select the owner of the order that should be querried
- `order_status` (String or null) : select the order staus that should be querried (pending,executed,canceled)

#### Usage

```javascript
getMarginOrders({"count_total", "limit", "reverse", "key"}, "order_type", "order_owner", status)
```

#### Example

```js
getMarginOrders(
  { count_total: true, limit: 10, reverse: false, key: null },
  "stop_loss",
  "elys12tzylat4udvjj56uuhu3vj2n4vgp7cf9fwna9w",
  "pending"
);
```

### 13. marginOpenEstimation (position,leverage,trading_asset,collateral,take_profit_price,user_address)

this function query an estimation on opening a MarginPosition.

#### Parameters

- `position` (String): The type of position for the margin order (e.g., "long", "short")
- `leverage` (String): The leverage for the margin position
- `trading_asset` (String): The trading asset
- `collateral` (Coin {`denom`: String, `amount`: String}) The amount that the user would like to send as a collateral
- `take_profit_price` (String): the take profit price for the open position
- `user_address` (String or null): user_address to calculate the discount that the user have access

#### Usage

```js
marginOpenEstimation(
  "position",
  "leverage",
  "trading_asset",
  "collateral",
  "take_profit_price",
  "user_address"
);
```

#### Example

```js
marginOpenEstimation(
  "long",
  "2.5",
  "ubtc",
  { denom: "uusdc", amount: "20000" },
  "1.5",
  "elys12tzylat4udvjj56uuhu3vj2n4vgp7cf9fwna9w"
);
```

### 14. stakeRequest (amount, asset, validator_address?)

you can use this function to stake uelys, ueden, uedenb and uusdc.

#### Parameters

- `amount` (uint64): The amount of token to stake
- `asset` (String): The denomination of the token to stake
- `validator_address?` (String): Validator address to stake. This is optional parameter which is only used for uelys staking.

#### Usage

```js
stake_request(
  "amount",
  "asset",
  "validator_address"?,
);
```

#### Example

```js
stake_request(
  3000000,
  "uelys",
  "elysvaloper17wc3s7am5qgjk4pm0k96kg6laxq8hkyq0dzq5n",
);
```

```js
stake_request(
  3000000,
  "ueden",
);
```

```js
stake_request(
  3000000,
  "uedenb",
);
```

```js
stake_request(
  3000000,
  "uusdc",
);
```

### 15. unstakeRequest (amount, asset, validator_address?)

you can use this function to unstake uelys, ueden, uedenb and uusdc.

#### Parameters

- `amount` (uint64): The amount of token to unstake
- `asset` (String): The denomination of the token to unstake
- `validator_address?` (String): Validator address to unstake. This is optional parameter which is only used for uelys unstaking.

#### Usage

```js
unstake_request(
  "amount",
  "asset",
  "validator_address"?,
);
```

#### Example

```js
unstake_request(
  3000000,
  "uelys",
  "elysvaloper17wc3s7am5qgjk4pm0k96kg6laxq8hkyq0dzq5n",
);
```

```js
unstake_request(
  3000000,
  "ueden",
);
```

```js
unstake_request(
  3000000,
  "uedenb",
);
```

```js
unstake_request(
  3000000,
  "uusdc",
);
```

### 16. elysRedelegationRequest (validator_src_address, validator_dst_address, amount)

you can use this function to re-delegate token.

#### Parameters

- `validator_src_address` (String): The source validator address.
- `validator_dst_address` (String): The destination validator address.
- `amount` (Coin): The amount of token to re-delegate.

#### Usage

```js
elys_redelegation_request(
  "validator_src_address",
  "validator_dst_address",
  "amount",
);
```

#### Example

```js
elys_redelegation_request(
  "elysvaloper12tzylat4udvjj56uuhu3vj2n4vgp7cf9pwcqcs",
  "elysvaloper17wc3s7am5qgjk4pm0k96kg6laxq8hkyq0dzq5n",
  { denom: "uelys", amount: "20000" },
);
```

### 17. elysCancelUnstakeRequest (validator_address, amount, creation_height)

you can use this function to cancel unbonding.

#### Parameters

- `validator_address` (String): The source validator address.
- `amount` (Coin): The amount of token to cancel un-stake.
- `creation_height` (int64): The block height that the bonding created.

#### Usage

```js
elys_cancel_unstake_request(
  "validator_address",
  "amount",
  "creation_height",
);
```

#### Example

```js
elys_cancel_unstake_request(
  "elysvaloper12tzylat4udvjj56uuhu3vj2n4vgp7cf9pwcqcs",
  { denom: "uelys", amount: "20000" },
  100000,
);
```

### 18. edenVestRequest (amount)

you can use this function to vest eden token

#### Parameters

- `amount` (Coin): The amount of eden to vest.

#### Usage

```js
eden_vest_request(
  "amount",
);
```

#### Example

```js
eden_vest_request(
  { denom: "uelys", amount: "20000" },
);
```

### 19. edenCancelVestRequest (amount)

you can use this function to cancel vesting of eden.

#### Parameters

- `amount` (Coin): The amount of eden to cancel vest.

#### Usage

```js
eden_cancel_vest_request(
  "amount",
);
```

#### Example

```js
eden_cancel_vest_request(
  { denom: "uelys", amount: "20000" },
);
```

### 20. claimRewardsRequest (amount)

you can use this function to claim rewards.

#### Parameters

- `withdraw_type` (i32): The program type to claim rewards.

#### Usage

```js
claim_rewards_request(
  "withdraw_type",
);
```

#### Example

```js
claim_rewards_request(
  1, // Earntype_UsdcProgram
);
```

#### Enum types
```
pub enum EarnType {
    AllProgram = 0,
    UsdcProgram = 1,
    ElysProgram = 2,
    EdenProgram = 3,
    EdenBProgram = 4,
}
```

### 21. claimValidatorCommissionRequest (validator_address)

you can use this function to claim validator's commission.

#### Parameters

- `validator_address` (String): The validator address to claim rewards.

#### Usage

```js
claim_validator_commission_request(
  "validator_address",
);
```

#### Example

```js
claim_validator_commission_request(
  "elysvaloper12tzylat4udvjj56uuhu3vj2n4vgp7cf9pwcqcs",
);
```

### 22. ammJoinPoolRequest (pool_id, max_amounts_in, share_amount_out, no_remaining)

you can use this function to join amm pool

#### Parameters

- `pool_id` (uint64): Pool id to join.
- `max_amounts_in` (Array<Coin>): The array of maximum tokens in.
- `share_amount_out` (Uint128): The amount of share tokens out.
- `no_remaining` (bool): The boolean flag that indicates that the amount of token in should be exactly the same amount of LP liquidity.

#### Usage

```js
amm_join_pool_request(
  "pool_id",
  "max_amounts_in",
  "share_amount_out",
  "no_remaining",
);
```

#### Example

```js
amm_join_pool_request(
  1,
  [{denom: "uelys", amount: "20000"}, {denom: "usdc", amount: "10000"}],
  10000,
  true,
);
```

### 23. ammExitPoolRequest (pool_id, min_amounts_out, share_amount_in, token_out_denom)

you can use this function to exit amm pool

#### Parameters

- `pool_id` (uint64): Pool id to exit.
- `min_amounts_out` (Array<Coin>): The array of maximum tokens out.
- `share_amount_in` (Uint128): The amount of share tokens in.
- `token_out_denom` (String): The out token denom.

#### Usage

```js
amm_exit_pool_request(
  "pool_id",
  "min_amounts_out",
  "share_amount_in",
  "token_out_denom",
);
```

#### Example

```js
amm_exit_pool_request(
  1,
  [{denom: "uelys", amount: "20000"}, {denom: "usdc", amount: "10000"}],
  10000,
  "uelys",
);
```

### 24. getLiquidityPools (pool_ids?, filter_type, pagination?)

you can use this function to exit amm pool

#### Parameters

- `pool_ids` (uint64): Pool id to exit.
- `filter_type` (Array<Coin>): The array of maximum tokens out.
- `pagination` (Uint128): The amount of share tokens in.

#### Usage

```js
get_liquidity_pools(
  "pool_ids"?,
  "filter_type",
  "pagination"?,
);
```

#### Example

```js
get_liquidity_pools(
  [],
  "filter_all,
  10000,
  "{"offset":0, "limit":10, "count_total": false, "reverse": true}}",
);
```

```js
get_liquidity_pools(
  [1,2],
  "filter_all,
  10000,
  "{"offset":1, "limit":10, "count_total": false, "reverse": true}}",
);
```

```js
get_liquidity_pools(
  "filter_all,
  10000,
  "{"offset":0, "limit":10, "count_total": false, "reverse": false}}}",
);
```
## Configuration

Before using these functions, you need to configure the following parameters in the script:

- `rpcEndpoint`: The URL of the blockchain RPC endpoint.
- `sender`: Information about the sender, including the mnemonic and address.
- `GASPRICE`: The gas price in the network's native token.
- `trade_shield_contract_addr`: The address of the CosmWasm contract with which you want to interact.

Make sure to replace these parameters with your specific network and contract details before running the functions.
