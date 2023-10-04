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

const trade_shield_contract_addr =
  "elys14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s3fsthx";

async function getOrder(order_id) {
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
      get_order: { order_id: order_id },
    }
  );
  console.log(`Result: `, result);
}

async function createOrder() {
  const gasPrice = GasPrice.fromString("0.05uelys");
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
    create_order: {
      order_amm_routes: [],
      order_price: { denom: "ueden", amount: "15" },
      order_type: "stop_loss",
    },
  };

  const create_order_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    "",
    coins("512", "uelys")
  );
  console.log("create_order_res:", create_order_res);
}

async function cancelOrder(order_id) {
  const gasPrice = GasPrice.fromString("0.05uelys");
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
    cancel_order: {
      order_id: order_id,
    },
  };

  const create_order_res = await sender_client.execute(
    sender.address,
    trade_shield_contract_addr,
    msg,
    executeFee,
    ""
  );
  console.log("create_order_res:", create_order_res);
}
