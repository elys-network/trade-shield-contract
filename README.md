# TradeShield Contract

TradeShield is a repository hosting a smart contract for implementing stop loss and limit sell functionalities on the Elys Network blockchain. This repository provides a robust and customizable solution for integrating advanced risk management mechanisms into decentralized applications.

## Specifications

The `docs` folder in this repository contains detailed specifications for implementing stop loss and limit sell functionalities using the TradeShield smart contract. It includes comprehensive documentation and guidelines to assist developers in understanding and utilizing these features effectively.

## Smart Contract Framework

TradeShield smart contracts are built using the CosmWasm framework, which provides a secure and efficient environment for executing WebAssembly-based smart contracts. These contracts are specifically designed for compatibility with Cosmos SDK 0.47 and the Elys Cosmos Modules.

## Features

### Spot Order

A spot order is fulfilled when the specified price set by the trader aligns with the prevailing market price, prompting the immediate execution of the purchase or sale of a financial asset.

- **Stop Loss Functionality**: Enable users to set automated orders that trigger when the asset's price reaches a specified lower limit, minimizing potential losses.
- **Limit Sell Functionality**: Allow users to set automated orders that execute when the asset's price reaches a specified upper limit, securing profits.
- **Limit Buy Fuctionality**: Allow users to set automated orders that execute when the limit price is reaches, securing profits.
- **Market Buy Fuctionality**: Allow users to set automated orders that will execute at market price

### Margin Order

A margin order involves trading an asset using funds provided by a third party. It allows users to trade larger positions with a smaller initial capital outlay.

- **Stop Loss Functionality**: Enable users to set automated orders that trigger when the asset's price reaches a specified lower limit, minimizing potential losses.
- **Limit Sell Functionality**: Allow users to set automated orders that execute when the asset's price reaches a specified upper limit, securing profits.
- **Limit Buy Fuctionality**: Allow users to set automated orders that execute when the limit price is reaches, securing profits.
- **Market Buy Fuctionality**: Allow users to set automated orders that will execute at market price

## Getting Started

To start using TradeShield, follow these steps:

1. Clone this repository to your local development environment.
2. Navigate to the `docs` folder to access the detailed specifications and guidelines for implementing stop loss and limit sell functionalities.
3. Ensure you have the required dependencies, including the CosmWasm framework, Cosmos SDK 0.47, and the Elys Cosmos Modules.
4. Review the example implementations provided in the repository to understand the integration process.
5. Customize and deploy the TradeShield smart contracts according to your specific project requirements.

## Deployment Steps for Elys Network

1. Install Ignite CLI
   Begin by installing the Ignite CLI tool by following the instructions provided in the [Ignite CLI documentation](https://docs.ignite.com/welcome/install).

2. Clone the Elys Repository
   Clone the [Elys repository](https://github.com/elys-network/elys/releases) from GitHub and carefully follow the installation instructions provided.

3. Modify `config.yml`
   Within the Elys repository, navigate to the root directory and locate the `config.yml` file. Modify the following lines:

- Change the `contract_addresses` value (line 38) to `["elys14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s3fsthx"]`.
- Modify the `broker_address` (line 439) to `"elys14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s3fsthx"`.

4. Start Ignite Node
   Open a separate terminal within the Elys repository and run the command:

```sh
ignite chain serve -r
```

Optionally, use `-v` to display the node logs.

5. Download the Contract
   Download the desired version of the contract from the [contract repository](https://github.com/elys-network/trade-shield-contract/releases).

6. Store Contract on Chain
   Run the following command to store the contract on the chain:

```sh
elysd tx wasm store path/to/account_history_contract.wasm  --from=treasury --keyring-backend=test --chain-id=elystestnet-1 --gas=auto --gas-adjustment=1.3 -y -b=sync
```

Ensure to replace `path/to/account_history_contract.wasm` with the actual path to the downloaded contract.

7. Instantiate Contract
   Execute the command below to instantiate the contract on the Elys network:

```sh
elysd tx wasm instantiate 1 '{}' --from=treasury --label "Contract" --chain-id=elystestnet-1 --gas=auto --gas-adjustment=1.3 -b=sync --keyring-backend=test --no-admin -y
```

## Contributing

We welcome contributions from the community to enhance TradeShield's functionality and usability. If you would like to contribute, please follow the guidelines outlined in the `CONTRIBUTING.md` file.

## License

TradeShield is released under the [Apache License](LICENSE). You are free to use, modify, and distribute this software as per the terms of the license.

## Issue Tracker

If you encounter any issues, bugs, or have any suggestions for improvement, please open an issue on the [Issue Tracker](https://github.com/elys-network/trade-shield-contract/issues). We appreciate your feedback and will address the concerns as soon as possible.
