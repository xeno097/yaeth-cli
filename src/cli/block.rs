use std::str::FromStr;

use crate::{cmd, context::CommandExecutionContext};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{BlockId, BlockNumber};

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

    Receipts(GetBlockTransactionCountArgs),
}

#[derive(Parser, Debug)]
#[command()]
pub struct BlockSubCommand {
    #[arg(long)]
    hash: Option<String>,

    #[arg(long)]
    number: Option<u64>,

    #[arg(long)]
    tag: Option<String>,

    #[command(subcommand)]
    command: BlockCommand,
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
    sub_command: BlockSubCommand,
) -> Result<(), anyhow::Error> {
    println!("{:#?}", sub_command);

    let BlockSubCommand {
        hash,
        number,
        tag,
        command,
    } = sub_command;

    let get_block = GetBlockTransactionCountArgs { hash, number, tag };

    match command {
        BlockCommand::Get(get_block_args) => {
            let include_tx = get_block_args.include_tx.unwrap_or_default();

            let _ = context.execute(cmd::block::get_block(
                context,
                get_block.try_into().unwrap(),
                include_tx,
            ));
        }
        BlockCommand::Number(_) => todo!(),
        // {
        //     let _ = context.execute(cmd::block::get_block_number(context));
        // }
        BlockCommand::Transaction(_transaction_command) => todo!(),
        // match transaction_command {
        //     BlockTransactionSubCommand::Count(get_block_transaction_count) => {
        //         let _ = context.execute(cmd::block::get_transaction_count(
        //             context,
        //             get_block_transaction_count,
        //         ));
        //     }
        // },
        BlockCommand::Uncle(_uncle_command) => todo!(),
        // match uncle_command {
        //     BlockTransactionSubCommand::Count(get_block_transaction_count) => {
        //         let _ = context.execute(cmd::block::get_uncle_block_count(
        //             context,
        //             get_block_transaction_count,
        //         ));
        //     }
        // },
        BlockCommand::Receipts(_receipt_command) => todo!(),
        // {
        // let _ = context.execute(cmd::block::get_block_receipts(context, receipt_command));
        // }
    }

    Ok(())
}
