use std::str::FromStr;

use crate::{cmd, context::CommandExecutionContext};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{BlockId, BlockNumber};

#[derive(Args, Debug)]
pub struct NoArgs;

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockCommand {
    /// Gets a block using the provided identifier  
    Get(GetBlockArgs),

    /// Gets the number of the most recent block
    Number(NoArgs),

    #[command(subcommand)]
    Transaction(BlockTransactionSubCommand),

    #[command(subcommand)]
    Uncle(BlockTransactionSubCommand),

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

enum GetBlockById {
    Hash(String),
    Tag(BlockTag),
    Number(u64),
    None,
}

impl GetBlockById {
    pub fn new(
        hash: Option<String>,
        number: Option<u64>,
        tag: Option<BlockTag>,
    ) -> Result<Self, anyhow::Error> {
        // Sanity check even if it shouldn't be possible because the check is performed by the cli
        if hash.is_some() && number.is_some()
            || hash.is_some() && tag.is_some()
            || number.is_some() && tag.is_some()
        {
            return Err(anyhow::anyhow!("Provided more than one block identifier"));
        }

        if let Some(hash) = hash {
            return Ok(Self::Hash(hash));
        }

        if let Some(block_number) = number {
            return Ok(Self::Number(block_number));
        }

        if let Some(tag) = tag {
            return Ok(Self::Tag(tag));
        }

        Ok(Self::None)
    }
}

#[derive(Debug, Clone)]
enum BlockTag {
    Latest,
    Finalized,
    Safe,
    Earliest,
    Pending,
}

// Used by clap's value_parser
impl FromStr for BlockTag {
    type Err = String;

    fn from_str(maybe_tag: &str) -> Result<Self, Self::Err> {
        match maybe_tag.to_lowercase().trim() {
            "latest" => Ok(BlockTag::Latest),
            "finalized" => Ok(BlockTag::Finalized),
            "safe" => Ok(BlockTag::Safe),
            "earliest" => Ok(BlockTag::Earliest),
            "pending" => Ok(BlockTag::Pending),
            _ => Err(format!("Received invalid block tag: {maybe_tag}")),
        }
    }
}

impl From<BlockTag> for BlockId {
    fn from(value: BlockTag) -> Self {
        let tag = match value {
            BlockTag::Latest => BlockNumber::Latest,
            BlockTag::Finalized => BlockNumber::Finalized,
            BlockTag::Safe => BlockNumber::Safe,
            BlockTag::Earliest => BlockNumber::Earliest,
            BlockTag::Pending => BlockNumber::Pending,
        };

        BlockId::Number(tag)
    }
}

impl TryFrom<GetBlockById> for BlockId {
    type Error = anyhow::Error;

    fn try_from(value: GetBlockById) -> Result<Self, Self::Error> {
        match value {
            GetBlockById::Hash(hash) => {
                Ok(BlockId::Hash(hash.parse().map_err(|_| {
                    anyhow::anyhow!("Invalid block hash format")
                })?))
            }
            GetBlockById::Tag(tag) => Ok(tag.into()),
            GetBlockById::Number(block_number) => {
                Ok(BlockId::Number(BlockNumber::Number(block_number.into())))
            }
            GetBlockById::None => Err(anyhow::anyhow!("Missing block identifier")),
        }
    }
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

    match command {
        BlockCommand::Get(get_block_args) => {
            let include_tx = get_block_args.include_tx.unwrap_or_default();

            let res = context.execute(cmd::block::get_block(
                context,
                get_block_by_id.try_into()?,
                include_tx,
            ))?;

            println!("{:#?}", res);
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
