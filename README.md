# yaeth-cli (yet another ethereum cli)

yaeth-cli is a command-line interface (CLI) tool written in Rust that serves as a wrapper around ethers-rs, and uses the clap crate for command-line argument parsing, enabling users to query the Ethereum blockchain from a terminal.

This is a toy project intended to further improve my Rust knowledge and explore the [Ethereum JSON-RPC  spec](https://ethereum.github.io/execution-apis/api-documentation/).

## Overview

```sh
An ether-rs wrapper to query the ethereum blockchain from a terminal

Usage: yaeth [OPTIONS] <COMMAND>

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

  -V, --version
```

## Installation

To try yaeth-cli, follow these steps:

1. Clone the repository to your local machine.
2. Install Rust and Cargo, if you haven't already, following the [guide](https://doc.rust-lang.org/book/ch01-01-installation.html).
3. Navigate to the cloned yaeth-cli directory in your terminal.
4. Run the following command to build and install yaeth-cli:

```sh
cargo install --path .
```

## Examples

Get transaction data:

```sh
yaeth --config-file=mainnet-config.json transaction --hash=0x79202697c177e951ea2bdfc283ef9a44108c41e2023cf56c4fd233a589da2e6a get
```

Query account balance:

```sh
yaeth --config-file=mainnet-config.json account --ens=vitalik.eth balance
```

Get block data:

```sh
yaeth --config-file=mainnet-config.json block --number=17081411 get
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

- [x] Gas
  - [x] eth_estimateGas
  - [x] eth_feeHistory
  - [x] eth_gasPrice
  - [x] eth_maxPriorityFeePerGas

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

Special thanks to the developers of [ethers-rs](https://github.com/gakonst/ethers-rs) and [clap](https://github.com/clap-rs/clap) for their work.
