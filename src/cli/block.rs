use crate::{
    cli::common::GetBlockArgs as GetBlockByIdArgs,
    cmd::block::{self, BlockKind},
    context::CommandExecutionContext,
};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{TransactionReceipt, U256, U64};

use super::common::NoArgs;

#[derive(Parser, Debug)]
#[command()]
pub struct BlockCommand {
    #[clap(flatten)]
    get_block_by_id: GetBlockByIdArgs,

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
    Block(BlockKind),
    Number(U64),
    Count(U256),
    TransactionReceipts(Vec<TransactionReceipt>),
    NotFound(),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: BlockCommand,
) -> Result<(), anyhow::Error> {
    let BlockCommand {
        get_block_by_id,
        command,
    } = sub_command;

    let res: BlockNamespaceResult = match command {
        BlockSubCommand::Get(GetBlockArgs { include_tx }) => context
            .execute(block::get_block(
                context,
                get_block_by_id.try_into()?,
                include_tx.unwrap_or_default(),
            ))?
            .map_or(
                BlockNamespaceResult::NotFound(),
                BlockNamespaceResult::Block,
            ),
        BlockSubCommand::Number(_) => context
            .execute(block::get_block_number(context))
            .map(BlockNamespaceResult::Number)?,
        BlockSubCommand::TransactionCount(_) => context
            .execute(block::get_transaction_count(
                context,
                get_block_by_id.try_into()?,
            ))?
            .map_or(
                BlockNamespaceResult::NotFound(),
                BlockNamespaceResult::Count,
            ),
        BlockSubCommand::UncleCount(_) => context
            .execute(block::get_uncle_block_count(
                context,
                get_block_by_id.try_into()?,
            ))
            .map(BlockNamespaceResult::Count)?,
        BlockSubCommand::Receipts(_) => context
            .execute(block::get_block_receipts(
                context,
                get_block_by_id.try_into()?,
            ))?
            .map_or(
                BlockNamespaceResult::NotFound(),
                BlockNamespaceResult::TransactionReceipts,
            ),
    };

    println!("{:#?}", res);

    Ok(())
}
