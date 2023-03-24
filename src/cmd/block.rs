use std::str::FromStr;

use crate::{config::CliConfig, context::CommandExecutionContext};
use clap::{Args, Subcommand};
use ethers::{
    providers::Middleware,
    types::{BlockId, BlockNumber},
};

#[derive(Subcommand, Debug)]
#[command()]
pub enum BlockCommand {
    Get(GetBlockArgs),
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

pub fn parse(
    context: &CommandExecutionContext,
    command: BlockCommand,
) -> Result<(), anyhow::Error> {
    match command {
        BlockCommand::Get(get_block_args) => {
            let _ = context.execute(get_block(context.config(), get_block_args));
        }
    }

    Ok(())
}

async fn get_block(config: &CliConfig, get_block_args: GetBlockArgs) -> Result<(), anyhow::Error> {
    let provider = ethers::providers::Provider::try_from(config.rpc_url()).unwrap();

    let block_id: BlockId = get_block_args.try_into().unwrap();

    let block = provider.get_block(block_id).await?;

    println!("{:#?}", block);

    Ok(())
}
