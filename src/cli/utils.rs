use crate::{
    cmd::utils::{self, SignTransactionData},
    context::CommandExecutionContext,
};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{Bytes, EIP1186ProofResponse, Signature, SyncingStatus, H160, H256, U256};
use serde::Serialize;

use super::common::{
    GetAccountArgs, GetBlockByIdArgs, NoArgs, TypedTransactionArgs, TypedTransactionParserError,
    TX_ARGS_FIELD_NAMES,
};

#[derive(Parser, Debug)]
#[command()]
pub struct UtilsCommand {
    #[command(subcommand)]
    command: UtilsSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum UtilsSubCommand {
    /// Gets the accounts known by the node
    Accounts(NoArgs),

    /// Gets the chain id from the node
    ChainId(NoArgs),

    /// Gets the EIP-1186 proof for the provided input
    Proof(GetProofArgs),

    /// Gets the ethereum protocol version
    ProtocolVersion(NoArgs),

    /// Signs the given transaction or data
    Sign(SignArgs),

    /// Gets the current sync status for the node
    SyncStatus(NoArgs),
}

#[derive(Args, Debug)]
pub struct GetProofArgs {
    #[clap(flatten)]
    get_account_by_id: GetAccountArgs,

    #[arg()]
    storage_locations: Vec<H256>,

    #[clap(flatten)]
    get_block_by_id: GetBlockByIdArgs,
}

#[derive(Args, Debug)]
pub struct SignArgs {
    #[clap(flatten)]
    get_account_by_id: GetAccountArgs,

    /// Raw byte data to sign
    #[clap(conflicts_with_all = TX_ARGS_FIELD_NAMES)]
    raw: Option<Bytes>,

    #[clap(flatten)]
    typed_tx: TypedTransactionArgs,
}

impl TryFrom<TypedTransactionArgs> for SignTransactionData {
    type Error = TypedTransactionParserError;

    fn try_from(tx: TypedTransactionArgs) -> Result<Self, Self::Error> {
        Ok(Self::Transaction(tx.try_into()?))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UtilsNamespaceResult {
    Accounts(Vec<H160>),
    ChainId(U256),
    Proof(EIP1186ProofResponse),
    ProtocolVersion(U256),
    Sign(Signature),
    SyncStatus(SyncingStatus),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: UtilsCommand,
) -> Result<UtilsNamespaceResult, anyhow::Error> {
    let node_provider = context.node_provider();

    let res: UtilsNamespaceResult = match sub_command.command {
        UtilsSubCommand::Accounts(_) => context
            .execute(utils::get_accounts(node_provider))
            .map(UtilsNamespaceResult::Accounts),
        UtilsSubCommand::ChainId(_) => context
            .execute(utils::get_chain_id(node_provider))
            .map(UtilsNamespaceResult::ChainId),
        UtilsSubCommand::Proof(GetProofArgs {
            get_account_by_id,
            storage_locations,
            get_block_by_id,
        }) => context
            .execute(utils::get_proof(
                node_provider,
                get_account_by_id.try_into()?,
                storage_locations,
                get_block_by_id.try_into().ok(),
            ))
            .map(UtilsNamespaceResult::Proof),
        UtilsSubCommand::ProtocolVersion(_) => context
            .execute(utils::get_protocol_version(node_provider))
            .map(UtilsNamespaceResult::ProtocolVersion),
        UtilsSubCommand::Sign(SignArgs {
            get_account_by_id,
            raw: data,
            typed_tx: tx,
        }) => context
            .execute(utils::sign(
                node_provider,
                get_account_by_id.try_into()?,
                data.map(SignTransactionData::Raw)
                    .map_or_else(|| tx.try_into(), Ok)?,
            ))
            .map(UtilsNamespaceResult::Sign),
        UtilsSubCommand::SyncStatus(_) => context
            .execute(utils::get_sync_status(node_provider))
            .map(UtilsNamespaceResult::SyncStatus),
    }?;

    Ok(res)
}
