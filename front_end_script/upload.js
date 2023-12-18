const {
  calculateFee,
  GasPrice,
  isMsgSubmitProposalEncodeObject,
} = require("@cosmjs/stargate");
const { DirectSecp256k1HdWallet, coins } = require("@cosmjs/proto-signing");
const {
  SigningCosmWasmClient,
  CosmWasmClient,
} = require("@cosmjs/cosmwasm-stargate");
const _ = require("fs");

// const rpcEndpoint = "https://osmosis-testnet-rpc.polkachu.com:443";
const rpcEndpoint = "http://localhost:26657";

const sender = {
  mnemonic:
    "olympic slide park figure frost benefit deer reform fly pull price airport submit monitor silk insect uphold convince pupil project ignore roof warfare slight",
  address: "elys12tzylat4udvjj56uuhu3vj2n4vgp7cf9fwna9w",
};

const GASPRICE = "0.05uelys";

const trade_shield_contract_addr =
  "elys14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s3fsthx";

async function getSpotOrder(order_id) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      get_spot_order: { order_id: order_id },
    }
  );
  console.log(`Result: `, result);
}

async function createSpotOrder(
  order_price,
  order_type,
  amount_send,
  denom_send,
  order_target_denom
) {
  const gasPrice = GasPrice.fromString(GASPRICE);
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const executeFee = calculateFee(300_000, gasPrice);
  const msg = {
    create_spot_order: {
      order_price: order_price,
      order_type: order_type,
      order_source_denom: denom_send,
      order_target_denom: order_target_denom,
    },
  };

  const create_spot_order_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    "",
    coins(amount_send, denom_send)
  );
  console.log("create_spot_order_res:", create_spot_order_res);
}

async function cancelSpotOrder(order_id) {
  const gasPrice = GasPrice.fromString(GASPRICE);
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const executeFee = calculateFee(300_000, gasPrice);
  const msg = {
    cancel_spot_order: {
      order_id: order_id,
    },
  };

  const create_spot_order_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    ""
  );
  console.log("create_spot_order_res:", create_spot_order_res);
}

async function cancelSpotOrders(order_ids, order_type, owner_address) {
  const gasPrice = GasPrice.fromString(GASPRICE);
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const executeFee = calculateFee(300_000, gasPrice);
  const msg = {
    cancel_spot_orders: {
      order_type: order_type,
      owner_address: owner_address,
      order_ids: order_ids,
    },
  };

  const create_spot_orders_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    ""
  );
  console.log("create_spot_orders_res:", create_spot_orders_res);
}

async function getMarginOrders(
  pagination,
  order_type,
  order_owner,
  order_status
) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      get_margin_orders: {
        pagination: pagination,
        order_type: order_type,
        order_owner: order_owner,
        order_status: order_status,
      },
    }
  );
  console.log(`Result: `, result);
}

async function getSpotOrders(
  pagination,
  order_type,
  order_owner,
  order_status
) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      get_spot_orders: {
        pagination: pagination,
        order_type: order_type,
        order_owner: order_owner,
        order_status: order_status,
      },
    }
  );
  console.log(`Result: `, result);
}

async function SwapEstimationByDenom(
  amount,
  denom_in,
  denom_out,
  user_address
) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      swap_estimation_by_denom: {
        amount: amount,
        denom_in: denom_in,
        denom_out: denom_out,
        user_address: user_address,
      },
    }
  );
  console.log(`Result: `, result);
}

async function createMarginOrder(
  position_type,
  collateral,
  leverage_value,
  trading_asset,
  take_profit_price,
  order_type,
  trigger_price,
  position_id
) {
  const gasPrice = GasPrice.fromString(GASPRICE);
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const executeFee = calculateFee(300_000, gasPrice);
  const msg = {
    create_margin_order: {
      position_type: position_type,
      leverage_value: leverage_value,
      trading_asset: trading_asset,
      position_id: position_id,
      take_profit_price: take_profit_price,
      order_type: order_type,
      trigger_price: trigger_price,
    },
  };
  let amount_send;

  if (collateral == null) {
    amount_send = [];
  } else {
    amount_send = coins(collateral.amount, collateral.denom);
  }

  const create_margin_order_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    "",
    amount_send
  );
  console.log("create_margin_order_res:", create_margin_order_res);
}

async function cancelMarginOrder(order_id) {
  const gasPrice = GasPrice.fromString(GASPRICE);
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const executeFee = calculateFee(300_000, gasPrice);
  const msg = {
    cancel_margin_order: {
      order_id: order_id,
    },
  };

  const create_margin_order_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    ""
  );
  console.log("create_margin_order_res:", create_margin_order_res);
}

async function getMarginOrder(order_id) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      get_margin_order: { order_id: order_id },
    }
  );
  console.log(`Result: `, result);
}

async function getMarginPosition(id, address) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      get_margin_position: { id: id, address: address },
    }
  );
  console.log(`Result: `, result);
}

async function getMarginPositions(pagination) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      get_margin_positions: { pagination: pagination },
    }
  );
  console.log(`Result: `, result);
}

async function marginOpenEstimation(
  position,
  leverage,
  trading_asset,
  collateral,
  take_profit_price,
  user_address
) {
  const sender_wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    sender.mnemonic,
    { prefix: "elys" }
  );
  const sender_client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    sender_wallet
  );
  const result = await sender_client.queryContractSmart(
    trade_shield_contract_addr,
    {
      margin_open_estimation: {
        position: position,
        leverage: leverage,
        trading_asset: trading_asset,
        collateral: collateral,
        take_profit_price: take_profit_price,
        user_address: user_address,
      },
    }
  );
  console.log(`Result: `, result);
}
