# README

This README provides documentation for the three main functions in the provided script, which interacts with a CosmWasm contract on a blockchain network.

## Functions

### 1. createOrder(order_amm_routes, order_price, order_type, amount_send, denom_send)

This function allows you to create a new order by sending a transaction to the CosmWasm contract.

#### Parameters

- `order_amm_routes` (Vec): The route for the AMM module to swap the token.
- `order_price` (Obj: {denom: String, amount: String}): The price at which you want to create the order.
- `order_type` (String): The type of the order (e.g., "stop_loss" or "take_profit").
- `amount_send` (String): The amount of cryptocurrency to send in the order.
- `denom_send` (String): The denomination of the cryptocurrency to send.

#### Usage

```javascript
createOrder(
  "your_amm_routes_here",
  "your_order_price_here",
  "order_type",
  "amount_to_send_here",
  "denom_to_send_here"
);
```

### 2. cancelOrder(order_id)

This function allows you to cancel an existing order by sending a transaction to the CosmWasm contract.

#### Parameters

- `order_id` (String): The unique identifier for the order you want to cancel.

#### Usage

```javascript
cancelOrder("your_order_id_here");
```

### 3. getOrder(order_id)

This function retrieves information about a specific order by querying a CosmWasm contract on the blockchain.

#### Parameters

- `order_id` (String): The unique identifier for the order you want to retrieve.

#### Usage

```javascript
getOrder("your_order_id_here");
```

## Configuration

Before using these functions, you need to configure the following parameters in the script:

- `rpcEndpoint`: The URL of the blockchain RPC endpoint.
- `sender`: Information about the sender, including the mnemonic and address.
- `GASPRICE`: The gas price in the network's native token.
- `trade_shield_contract_addr`: The address of the CosmWasm contract with which you want to interact.

Make sure to replace these parameters with your specific network and contract details before running the functions.
