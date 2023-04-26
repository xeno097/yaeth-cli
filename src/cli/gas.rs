use crate::{cmd, context::CommandExecutionContext};

use super::common::{GetBlockByIdArgs, NoArgs, TypedTransactionArgs};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{FeeHistory, U256};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command()]
pub struct GasCommand {
    #[command(subcommand)]
    command: GasSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum GasSubCommand {
    /// Estimates the gas used by the provided transaction
    Estimate(EstimateGasArgs),

    /// Gets the transaction base fee per gas and effective priority fee per gas for the specified block range
    History(GetFeeHistoryArgs),

    /// Gets the current estimated gas price
    Price(NoArgs),

    /// Gets the current estimated max priority gas fee
    Fee(NoArgs),
}

#[derive(Args, Debug)]
pub struct EstimateGasArgs {
    // Typed Tx args
    #[clap(flatten)]
    typed_tx: TypedTransactionArgs,

    // Block id args
    #[clap(flatten)]
    get_block_by_id: GetBlockByIdArgs,
}

#[derive(Args, Debug)]
pub struct GetFeeHistoryArgs {
    /// The number of blocks to include in the requested range
    #[clap()]
    count: U256,

    /// The highest block of the requested range
    #[clap(flatten)]
    last_block: GetBlockByIdArgs,

    /// A monotonically increasing list of percentiles values to use to sort transactions based on the gas consumed
    #[clap()]
    percentiles: Vec<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GasNamespaceResult {
    Estimate(U256),
    Price(U256),
    Fee(U256),
    GetFeeHistory(Option<FeeHistory>),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: GasCommand,
) -> Result<GasNamespaceResult, anyhow::Error> {
    let node_provider = context.node_provider();

    let res: GasNamespaceResult = match sub_command.command {
        GasSubCommand::Estimate(EstimateGasArgs {
            get_block_by_id,
            typed_tx,
        }) => context
            .execute(cmd::gas::estimate_gas(
                node_provider,
                typed_tx.try_into()?,
                get_block_by_id.try_into().ok(),
            ))
            .map(GasNamespaceResult::Estimate),
        GasSubCommand::History(GetFeeHistoryArgs {
            count,
            last_block,
            percentiles,
        }) => context
            .execute(cmd::gas::get_fee_history(
                node_provider,
                count,
                last_block.try_into()?,
                percentiles,
            ))
            .map(GasNamespaceResult::GetFeeHistory),
        GasSubCommand::Price(_) => context
            .execute(cmd::gas::gas_price(node_provider))
            .map(GasNamespaceResult::Price),
        GasSubCommand::Fee(_) => context
            .execute(cmd::gas::get_max_priority_fee(node_provider))
            .map(GasNamespaceResult::Fee),
    }?;

    Ok(res)
}
