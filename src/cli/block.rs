use crate::{cli::common::GetBlockById, cmd::block, context::CommandExecutionContext};
use clap::{command, Args, Parser, Subcommand};

use super::common::{BlockTag, NoArgs};

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockCommand {
    /// Gets a block using the provided identifier  
    Get(GetBlockArgs),

    /// Gets the number of the most recent block
    Number(NoArgs),

    /// Collection of transaction related operations for the block with the provided identifier
    #[command(subcommand)]
    Transaction(BlockTransactionSubCommand),

    /// Collection of uncle blocks related operations for the block with the provided identifier
    #[command(subcommand)]
    Uncle(BlockTransactionSubCommand),

    /// Gets the transaction receipts for the block with the provided identifier
    Receipts(NoArgs),
}

#[derive(Parser, Debug)]
#[command()]
pub struct BlockSubCommand {
    #[arg(long, exclusive = true)]
    hash: Option<String>,

    #[arg(long, exclusive = true)]
    number: Option<u64>,

    #[arg(long, exclusive = true)]
    tag: Option<BlockTag>,

    #[command(subcommand)]
    command: BlockCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockTransactionSubCommand {
    /// Gets the number of transactions for the block
    Count(NoArgs),
}

#[derive(Args, Debug)]
pub struct GetBlockArgs {
    #[arg(long)]
    include_tx: Option<bool>,
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: BlockSubCommand,
) -> Result<(), anyhow::Error> {
    let BlockSubCommand {
        hash,
        number,
        tag,
        command,
    } = sub_command;

    let get_block_by_id = GetBlockById::new(hash, number, tag)?;

    let res = match command {
        BlockCommand::Get(get_block_args) => context.execute(block::get_block(
            context,
            get_block_by_id.into(),
            get_block_args.include_tx.unwrap_or_default(),
        ))?,
        BlockCommand::Number(_) => context.execute(block::get_block_number(context))?,
        BlockCommand::Transaction(transaction_command) => match transaction_command {
            BlockTransactionSubCommand::Count(_) => context.execute(
                block::get_transaction_count(context, get_block_by_id.into()),
            )?,
        },
        BlockCommand::Uncle(uncle_command) => match uncle_command {
            BlockTransactionSubCommand::Count(_) => context.execute(
                block::get_uncle_block_count(context, get_block_by_id.into()),
            )?,
        },
        BlockCommand::Receipts(_) => {
            context.execute(block::get_block_receipts(context, get_block_by_id.into()))?
        }
    };

    println!("{:#?}", res);

    Ok(())
}
