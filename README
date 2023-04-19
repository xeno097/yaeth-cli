# yaeth-cli (yet another ethereum cli)

yaeth-cli is a command-line interface (CLI) tool written in Rust that serves as a wrapper around ethers-rs, leveraging the clap crate for command-line argument parsing, enabling users to query the Ethereum blockchain from a terminal.

## Overview

```sh
An ether-rs wrapper to query the ethereum blockchain from a terminal

Usage: yaeth-cli [OPTIONS] <COMMAND>

Commands:
  block
          Execute block related operations
  account
          Execute account related operations
  transaction
          Execute transaction related operations
  event
          Execute event related operations
  gas
          Execute gas related operations
  utils
          Collection of utils

Options:
  -p, --priv-key <PRIV_KEY>
          Private key to use for signing transactions

  -r, --rpc-url <RPC_URL>
          Rpc url to send requests to

  -o, --out <OUT>
          Output format for the cli result
          
          [default: console]

          Possible values:
          - console: Output the cli result to the terminal
          - json:    Output the cli result to a json file

  -f, --file <FILE>
          Optional name for the output file
          
          [default: out]

  -c, --config-file <CONFIG_FILE>
          Optional configuration file

  -h, --help
          Print help (see a summary with '-h')
```

## Installation

To try yaeth-cli, follow these steps:

1. Clone the repository to your local machine.
2. Install Rust and Cargo, the Rust package manager, if you haven't already.
3. Navigate to the cloned yaeth-cli directory in your terminal.
4. Run the following command to build and install yaeth-cli:

```sh
cargo install --path .
```

## Examples

Get transaction data:

```sh
yaeth-cli --config-file=mainnet-config.json transaction --hash=0x79202697c177e951ea2bdfc283ef9a44108c41e2023cf56c4fd233a589da2e6a get
```

Query account balance:

```sh
yaeth-cli --config-file=mainnet-config.json account --ens=vitalik.eth balance
```

## Work in progress

- [x] Block:
  - [x] eth_blockNumber
  - [x] eth_getBlockByHash
  - [x] eth_getBlockByNumber
  - [x] eth_getBlockReceipts
  - [x] eth_getBlockTransactionCountByHash
  - [x] eth_getBlockTransactionCountByNumber
  - [x] eth_getUncleCountByBlockHash
  - [x] eth_getUncleCountByBlockNumber

- [x] Transaction
  - [x] eth_call
  - [x] eth_getTransactionByBlockHashAndIndex
  - [x] eth_getTransactionByBlockNumberAndIndex
  - [x] eth_getTransactionByHash
  - [x] eth_getTransactionReceipt
  - [x] eth_sendRawTransaction
  - [x] eth_sendTransaction
  
- [x] Account
  - [x] eth_getBalance
  - [x] eth_getCode
  - [x] eth_getStorageAt
  - [x] eth_getTransactionCount

- [ ] Gas
  - [ ] eth_estimateGas
  - [ ] eth_feeHistory
  - [ ] eth_gasPrice
  - [ ] eth_maxPriorityFeePerGas

- [ ] Utils
  - [ ] eth_accounts
  - [ ] eth_chainId
  - [ ] eth_coinbase
  - [ ] eth_getProof
  - [ ] eth_getRootHash
  - [ ] eth_hashrate
  - [ ] eth_mining
  - [ ] eth_protocolVersion
  - [ ] eth_sign
  - [ ] eth_syncing

- [ ] Event / Logs
  - [ ] eth_getFilterChanges
  - [ ] eth_getFilterLogs
  - [ ] eth_getLogs
  - [ ] eth_newBlockFilter
  - [ ] eth_newFilter
  - [ ] eth_newPendingTransactionFilter
  - [ ] eth_pendingTransactions
  - [ ] eth_uninstallFilter

## License

yaeth-cli is released under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgements

yaeth-cli is built on top of the ethers-rs library, which is a powerful Rust library for interacting with the Ethereum blockchain. It also uses the clap crate, a powerful and flexible command-line argument parsing library for Rust, to provide a user-friendly command-line interface. Special thanks to the developers of ethers-rs and clap for their excellent work and contributions to the Rust and Ethereum communities.
