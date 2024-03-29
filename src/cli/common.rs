use clap::{builder::PossibleValue, Args, ValueEnum};
use ethers::types::{
    Address, BlockId, BlockNumber, Bytes, NameOrAddress, TransactionRequest, H160, H256, U256, U64,
};
use serde::Serializer;
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

impl ValueEnum for BlockTag {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Earliest,
            Self::Finalized,
            Self::Latest,
            Self::Pending,
            Self::Safe,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            BlockTag::Latest => {
                PossibleValue::new("latest").help("Latest block added to the blockchain")
            }
            BlockTag::Finalized => PossibleValue::new("finalized")
                .help("Block accepted as part of the canonical blockchain"),
            BlockTag::Safe => PossibleValue::new("safe")
                .help("Block that received 2/3 attestation from validators"),
            BlockTag::Earliest => PossibleValue::new("earliest").help("Genesis block"),
            BlockTag::Pending => {
                PossibleValue::new("pending").help("Block not yet part of the blockchain")
            }
        })
    }
}

impl From<BlockTag> for BlockNumber {
    fn from(value: BlockTag) -> Self {
        match value {
            BlockTag::Latest => BlockNumber::Latest,
            BlockTag::Finalized => BlockNumber::Finalized,
            BlockTag::Safe => BlockNumber::Safe,
            BlockTag::Earliest => BlockNumber::Earliest,
            BlockTag::Pending => BlockNumber::Pending,
        }
    }
}

impl From<BlockTag> for BlockId {
    fn from(value: BlockTag) -> Self {
        BlockId::Number(value.into())
    }
}

pub const GET_BLOCK_BY_ID_ARG_GROUP_NAME: &str = "block_by_id";

#[derive(Args, Debug)]
pub struct GetBlockByIdArgs {
    /// Hash of the target block
    #[arg(group=GET_BLOCK_BY_ID_ARG_GROUP_NAME, long, value_name = "BLOCK_HASH",conflicts_with_all(["number","tag"]))]
    hash: Option<H256>,

    /// Number of the target block
    #[arg(group=GET_BLOCK_BY_ID_ARG_GROUP_NAME,long, value_name = "BLOCK_NUMBER", conflicts_with_all(["hash","tag"]))]
    number: Option<u64>,

    /// Tag of the target block
    #[arg(group=GET_BLOCK_BY_ID_ARG_GROUP_NAME,long, value_name = "BLOCK_TAG", conflicts_with_all(["hash","number"]))]
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

pub fn parse_not_found<S>(s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_none()
}

#[derive(Args, Debug)]
pub struct TypedTransactionArgs {
    /// Address of the account from which the transaction will be sent
    #[arg(long)]
    from: Option<Address>,

    /// Address of the account to send the transaction to
    #[arg(long, conflicts_with = "ens_to")]
    to: Option<Address>,

    /// Ens name of the account to send the transaction to
    #[arg(long)]
    ens_to: Option<String>,

    #[arg(long)]
    gas: Option<U256>,

    #[arg(long)]
    gas_price: Option<U256>,

    /// Amount of Eth to send
    #[arg(long)]
    value: Option<U256>,

    /// Calldata to send to the target account
    #[arg(long)]
    data: Option<Bytes>,

    #[arg(long)]
    nonce: Option<U256>,

    #[arg(long)]
    chain_id: Option<U64>,
}

pub const TX_ARGS_FIELD_NAMES: [&str; 9] = [
    "from",
    "to",
    "ens_to",
    "gas",
    "gas_price",
    "value",
    "data",
    "nonce",
    "chain_id",
];

#[derive(Error, Debug)]
pub enum TypedTransactionParserError {
    #[error("Provided both ens and address")]
    ConflictingTransactionReceiver,
}

impl TryFrom<TypedTransactionArgs> for TransactionRequest {
    type Error = TypedTransactionParserError;

    fn try_from(value: TypedTransactionArgs) -> Result<Self, Self::Error> {
        let TypedTransactionArgs {
            from,
            to,
            ens_to,
            gas,
            gas_price,
            value,
            data,
            nonce,
            chain_id,
        } = value;

        let mut tx = TransactionRequest::new();

        if ens_to.is_some() && to.is_some() {
            return Err(Self::Error::ConflictingTransactionReceiver);
        }

        if let Some(from) = from {
            tx = tx.from(from)
        }

        if let Some(to) = to {
            tx = tx.to(to)
        }

        if let Some(ens) = ens_to {
            tx = tx.to(ens)
        }

        if let Some(gas) = gas {
            tx = tx.gas(gas)
        }

        if let Some(gas_price) = gas_price {
            tx = tx.gas_price(gas_price)
        }

        if let Some(value) = value {
            tx = tx.value(value)
        }

        if let Some(data) = data {
            tx = tx.data(data)
        }

        if let Some(nonce) = nonce {
            tx = tx.nonce(nonce)
        }

        if let Some(chain_id) = chain_id {
            tx = tx.chain_id(chain_id)
        }

        Ok(tx)
    }
}

#[derive(Args, Debug)]
pub struct GetAccountArgs {
    /// Ethereum address for the account
    #[arg(long, conflicts_with = "ens", required_unless_present = "ens")]
    address: Option<H160>,

    /// Ens name for the account
    #[arg(long)]
    ens: Option<String>,
}

#[derive(Error, Debug)]
pub enum GetAccountParserError {
    #[error("Provided multiple account identifiers. Either an ens or address must be provided.")]
    ConflictingAccountId,

    #[error("Missing account identifier. An ens or address must be provided.")]
    MissingAccountId,
}

impl TryFrom<GetAccountArgs> for NameOrAddress {
    type Error = GetAccountParserError;

    fn try_from(GetAccountArgs { address, ens }: GetAccountArgs) -> Result<Self, Self::Error> {
        // Sanity check
        if address.is_some() && ens.is_some() {
            return Err(Self::Error::ConflictingAccountId);
        }

        if let Some(address) = address {
            return Ok(NameOrAddress::Address(address));
        };

        if let Some(ens) = ens {
            return Ok(NameOrAddress::Name(ens));
        };

        Err(Self::Error::MissingAccountId)
    }
}
