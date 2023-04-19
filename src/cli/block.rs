use crate::{
    cli::common::GetBlockByIdArgs,
    cmd::block::{self, BlockKind},
    context::CommandExecutionContext,
};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{TransactionReceipt, U256, U64};
use serde::Serialize;

use super::common::{parse_not_found, NoArgs};

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

    /// Gets the number of transaction in the block with the provided identifier
    TransactionCount(NoArgs),

    /// Gets the number of uncle blocks in the block with the provided identifier
    UncleCount(NoArgs),

    /// Gets the transaction receipts for the block with the provided identifier
    Receipts(NoArgs),
}

#[derive(Args, Debug)]
pub struct GetBlockArgs {
    /// Indicates if transactions should be included when getting block
    #[arg(long)]
    include_tx: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockNamespaceResult {
    Block(BlockKind),
    Number(U64),
    Count(U256),
    TransactionReceipts(Vec<TransactionReceipt>),
    #[serde(serialize_with = "parse_not_found", rename = "block")]
    NotFound(),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: BlockCommand,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    let BlockCommand {
        get_block_by_id,
        command,
    } = sub_command;

    let node_provider = context.node_provider();

    let res: BlockNamespaceResult = match command {
        BlockSubCommand::Get(GetBlockArgs { include_tx }) => context
            .execute(block::get_block(
                node_provider,
                get_block_by_id.try_into()?,
                include_tx.unwrap_or_default(),
            ))?
            .map_or(
                BlockNamespaceResult::NotFound(),
                BlockNamespaceResult::Block,
            ),
        BlockSubCommand::Number(_) => context
            .execute(block::get_block_number(node_provider))
            .map(BlockNamespaceResult::Number)?,
        BlockSubCommand::TransactionCount(_) => context
            .execute(block::get_transaction_count(
                node_provider,
                get_block_by_id.try_into()?,
            ))?
            .map_or(
                BlockNamespaceResult::NotFound(),
                BlockNamespaceResult::Count,
            ),
        BlockSubCommand::UncleCount(_) => context
            .execute(block::get_uncle_block_count(
                node_provider,
                get_block_by_id.try_into()?,
            ))
            .map(BlockNamespaceResult::Count)?,
        BlockSubCommand::Receipts(_) => context
            .execute(block::get_block_receipts(
                node_provider,
                get_block_by_id.try_into()?,
            ))?
            .map_or(
                BlockNamespaceResult::NotFound(),
                BlockNamespaceResult::TransactionReceipts,
            ),
    };

    Ok(res)
}
