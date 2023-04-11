use std::str::FromStr;

use clap::Args;
use ethers::types::{BlockId, BlockNumber, H256};
use thiserror::Error;

#[derive(Args, Debug)]
pub struct NoArgs;

#[derive(Debug, Clone)]
pub enum BlockTag {
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

#[derive(Args, Debug)]
pub struct GetBlockByIdArgs {
    /// Hash of the target block
    #[arg(long, value_name = "BLOCK_HASH",conflicts_with_all(["number","tag"]))]
    hash: Option<H256>,

    /// Number of the target block
    #[arg(long, value_name = "BLOCK_NUMBER", conflicts_with_all(["hash","tag"]))]
    number: Option<u64>,

    /// Tag of the target block
    #[arg(long, value_name = "BLOCK_TAG", conflicts_with_all(["hash","number"]))]
    tag: Option<BlockTag>,
}

#[derive(Error, Debug)]
pub enum BlockIdParserError {
    #[error("Missing block identifier. A block tag, number or hash must be provided.")]
    MissingBlockId,

    #[error(
        "Provided multiple block identifiers. Only a block tag, number or hash must be provided."
    )]
    ConflictingBlockId,
}

impl TryFrom<GetBlockByIdArgs> for BlockId {
    type Error = BlockIdParserError;

    fn try_from(value: GetBlockByIdArgs) -> Result<Self, Self::Error> {
        let GetBlockByIdArgs { hash, number, tag } = value;

        let check1 = hash.is_some() && number.is_some();
        let check2 = hash.is_some() && tag.is_some();
        let check3 = number.is_some() && tag.is_some();

        if check1 || check2 || check3 {
            return Err(Self::Error::ConflictingBlockId);
        }

        if let Some(hash) = hash {
            return Ok(Self::Hash(hash));
        }

        if let Some(number) = number {
            return Ok(Self::Number(BlockNumber::Number(number.into())));
        }

        if let Some(tag) = tag {
            return Ok(tag.into());
        }

        Err(Self::Error::MissingBlockId)
    }
}
