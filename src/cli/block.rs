use crate::{
    cli::common::GetBlockById,
    cmd::block::{self, BlockDataResult, BlockKind},
    context::CommandExecutionContext,
};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{Block, Transaction, TransactionReceipt, H256, U256, U64};

use super::common::{BlockTag, NoArgs};

#[derive(Parser, Debug)]
#[command()]
pub struct BlockCommand {
    #[arg(long, exclusive = true)]
    hash: Option<String>,

    #[arg(long, exclusive = true)]
    number: Option<u64>,

    #[arg(long, exclusive = true)]
    tag: Option<BlockTag>,

    #[command(subcommand)]
    command: BlockSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockSubCommand {
    /// Gets a block using the provided identifier  
    Get(GetBlockArgs),

    /// Gets the number of the most recent block
    Number(NoArgs),

    /// Collection of transaction related operations for the block with the provided identifier
    TransactionCount(NoArgs),

    /// Collection of uncle blocks related operations for the block with the provided identifier
    UncleCount(NoArgs),

    /// Gets the transaction receipts for the block with the provided identifier
    Receipts(NoArgs),
}

#[derive(Args, Debug)]
pub struct GetBlockArgs {
    #[arg(long)]
    include_tx: Option<bool>,
}

#[derive(Debug)]
pub enum BlockNamespaceResult {
    Block(Block<H256>),
    BlockWithTransaction(Block<Transaction>),
    Number(U64),
    Count(U256),
    TransactionReceipts(Vec<TransactionReceipt>),
    NotFound(),
}

impl From<Block<H256>> for BlockNamespaceResult {
    fn from(value: Block<H256>) -> Self {
        Self::Block(value)
    }
}

impl From<Block<Transaction>> for BlockNamespaceResult {
    fn from(value: Block<Transaction>) -> Self {
        Self::BlockWithTransaction(value)
    }
}

impl From<U256> for BlockNamespaceResult {
    fn from(value: U256) -> Self {
        Self::Count(value)
    }
}

impl From<U64> for BlockNamespaceResult {
    fn from(value: U64) -> Self {
        Self::Number(value)
    }
}

impl From<BlockKind> for BlockNamespaceResult {
    fn from(value: BlockKind) -> Self {
        match value {
            BlockKind::RawBlock(block) => Self::Block(block),
            BlockKind::BlockWithTransaction(block) => Self::BlockWithTransaction(block),
        }
    }
}

impl From<Vec<TransactionReceipt>> for BlockNamespaceResult {
    fn from(value: Vec<TransactionReceipt>) -> Self {
        BlockNamespaceResult::TransactionReceipts(value)
    }
}

impl<T: Into<BlockNamespaceResult>> From<BlockDataResult<T>> for BlockNamespaceResult {
    fn from(value: BlockDataResult<T>) -> Self {
        match value {
            BlockDataResult::Data(data) => data.into(),
            BlockDataResult::NotFound() => BlockNamespaceResult::NotFound(),
        }
    }
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: BlockCommand,
) -> Result<(), anyhow::Error> {
    let BlockCommand {
        hash,
        number,
        tag,
        command,
    } = sub_command;

    let get_block_by_id = GetBlockById::new(hash, number, tag)?;

    let res: BlockNamespaceResult = match command {
        BlockSubCommand::Get(get_block_args) => context
            .execute(block::get_block(
                context,
                get_block_by_id.into(),
                get_block_args.include_tx.unwrap_or_default(),
            ))?
            .into(),
        BlockSubCommand::Number(_) => context.execute(block::get_block_number(context))?.into(),
        BlockSubCommand::TransactionCount(_) => context
            .execute(block::get_transaction_count(
                context,
                get_block_by_id.into(),
            ))?
            .into(),
        BlockSubCommand::UncleCount(_) => context
            .execute(block::get_uncle_block_count(
                context,
                get_block_by_id.into(),
            ))?
            .into(),
        BlockSubCommand::Receipts(_) => context
            .execute(block::get_block_receipts(context, get_block_by_id.into()))?
            .into(),
    };

    println!("{:#?}", res);

    Ok(())
}
