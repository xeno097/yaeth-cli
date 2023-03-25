use std::str::FromStr;

use crate::context::CommandExecutionContext;
use clap::{Args, Subcommand};
use ethers::{
    providers::Middleware,
    types::{BlockId, BlockNumber},
};

#[derive(Args, Debug)]
pub struct NoArgs;

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockCommand {
    /// Gets a block by identifier  
    Get(GetBlockArgs),

    /// Gets the number of the most recent block
    Number(NoArgs),

    #[command(subcommand)]
    Transaction(BlockTransactionSubCommand),

    #[command(subcommand)]
    Uncle(BlockTransactionSubCommand),
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockTransactionSubCommand {
    /// Gets the number of transactions for the block
    Count(GetBlockTransactionCountArgs),
}

#[derive(Args, Debug)]
pub struct GetBlockArgs {
    #[arg(long)]
    hash: Option<String>,

    #[arg(long)]
    number: Option<u64>,

    #[arg(long)]
    tag: Option<String>,

    #[arg(long)]
    include_tx: Option<bool>,
}

#[derive(Args, Debug)]
pub struct GetBlockTransactionCountArgs {
    #[arg(long)]
    hash: Option<String>,

    #[arg(long)]
    number: Option<u64>,

    #[arg(long)]
    tag: Option<String>,
}

impl TryFrom<GetBlockArgs> for BlockId {
    type Error = String;

    fn try_from(value: GetBlockArgs) -> Result<Self, Self::Error> {
        if value.hash.is_some() {
            return Ok(BlockId::Hash(
                value
                    .hash
                    .unwrap()
                    .parse()
                    .map_err(|_| "Invalid block hash format")?,
            ));
        }

        if value.number.is_some() {
            return Ok(BlockId::Number(BlockNumber::Number(
                value.number.unwrap().into(),
            )));
        }

        if value.tag.is_some() {
            // TODO enforce tag to be a block tag and not a number even if the underlying type supports that
            return Ok(BlockId::Number(
                BlockNumber::from_str(&value.tag.unwrap())
                    .map_err(|_| "Failed to parse block tag")?,
            ));
        }

        Err(String::from("Failed to parse blcok identifier"))
    }
}

impl TryFrom<GetBlockTransactionCountArgs> for BlockId {
    type Error = String;

    fn try_from(value: GetBlockTransactionCountArgs) -> Result<Self, Self::Error> {
        if value.hash.is_some() {
            return Ok(BlockId::Hash(
                value
                    .hash
                    .unwrap()
                    .parse()
                    .map_err(|_| "Invalid block hash format")?,
            ));
        }

        if value.number.is_some() {
            return Ok(BlockId::Number(BlockNumber::Number(
                value.number.unwrap().into(),
            )));
        }

        if value.tag.is_some() {
            // TODO enforce tag to be a block tag and not a number even if the underlying type supports that
            return Ok(BlockId::Number(
                BlockNumber::from_str(&value.tag.unwrap())
                    .map_err(|_| "Failed to parse block tag")?,
            ));
        }

        Err(String::from("Failed to parse blcok identifier"))
    }
}

pub fn parse(
    context: &CommandExecutionContext,
    command: BlockCommand,
) -> Result<(), anyhow::Error> {
    match command {
        BlockCommand::Get(get_block_args) => {
            let _ = context.execute(get_block(context, get_block_args));
        }
        BlockCommand::Number(_) => {
            let _ = context.execute(get_block_number(context));
        }
        BlockCommand::Transaction(transaction_command) => match transaction_command {
            BlockTransactionSubCommand::Count(get_block_transaction_count) => {
                let _ =
                    context.execute(get_transaction_count(context, get_block_transaction_count));
            }
        },
        BlockCommand::Uncle(uncle_command) => match uncle_command {
            BlockTransactionSubCommand::Count(get_block_transaction_count) => {
                let _ =
                    context.execute(get_uncle_block_count(context, get_block_transaction_count));
            }
        },
    }

    Ok(())
}

// eth_getBlockByHash || eth_getBlockByNumber
async fn get_block(
    context: &CommandExecutionContext,
    get_block_args: GetBlockArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_args.try_into().unwrap();

    let block = context.node_provider().get_block(block_id).await?;

    println!("{:#?}", block);

    Ok(())
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
async fn get_transaction_count(
    context: &CommandExecutionContext,
    get_block_transaction_count: GetBlockTransactionCountArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_transaction_count.try_into().unwrap();

    let block = context.node_provider().get_block(block_id).await?;

    let transaction_count = block.unwrap().transactions.len();

    println!("{:#?}", transaction_count);

    Ok(())
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
async fn get_uncle_block_count(
    context: &CommandExecutionContext,
    get_block_transaction_count: GetBlockTransactionCountArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_transaction_count.try_into().unwrap();

    let block = context.node_provider().get_uncle_count(block_id).await?;

    println!("{:#?}", block);

    Ok(())
}

// eth_blockNumber
async fn get_block_number(context: &CommandExecutionContext) -> Result<(), anyhow::Error> {
    let block_number = context.node_provider().get_block_number().await?;

    println!("{:#?}", block_number);

    Ok(())
}

// TODO: implement protype function for this method
// eth_getBlockReceipts
