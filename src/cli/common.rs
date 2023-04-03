use std::str::FromStr;

use clap::Args;
use ethers::types::{BlockId, BlockNumber};

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

pub struct GetBlockById(BlockId);

impl GetBlockById {
    pub fn new(
        hash: Option<String>,
        number: Option<u64>,
        tag: Option<BlockTag>,
    ) -> Result<Self, anyhow::Error> {
        let check1 = hash.is_some() && number.is_some();
        let check2 = hash.is_some() && tag.is_some();
        let check3 = number.is_some() && tag.is_some();

        if check1 || check2 || check3 {
            return Err(anyhow::anyhow!("Provided more than one block identifier"));
        }

        if let Some(hash) = hash {
            return Ok(Self(BlockId::Hash(
                hash.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid block hash format"))?,
            )));
        }

        if let Some(block_number) = number {
            return Ok(Self(BlockId::Number(BlockNumber::Number(
                block_number.into(),
            ))));
        }

        if let Some(tag) = tag {
            return Ok(Self(tag.into()));
        }

        Ok(Self(BlockId::Number(BlockNumber::Latest)))
    }
}

impl From<GetBlockById> for BlockId {
    fn from(value: GetBlockById) -> Self {
        value.0
    }
}
