extern crate docopt;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate lib;

mod get_cli_args;
mod get_versions;
mod usage_info;

use lib::{
    btc_on_eth::{
        debug_add_multiple_utxos,
        debug_clear_all_utxos,
        debug_consolidate_utxos,
        debug_get_all_db_keys,
        debug_get_all_utxos,
        debug_get_child_pays_for_parent_btc_tx,
        debug_get_fee_withdrawal_tx,
        debug_get_key_from_db,
        debug_get_signed_erc777_change_pnetwork_tx,
        debug_get_signed_erc777_proxy_change_pnetwork_by_proxy_tx,
        debug_get_signed_erc777_proxy_change_pnetwork_tx,
        debug_maybe_add_utxo_to_db,
        debug_put_btc_on_eth_peg_in_basis_points_in_db,
        debug_put_btc_on_eth_peg_out_basis_points_in_db,
        debug_remove_utxo,
        debug_reprocess_btc_block,
        debug_reprocess_eth_block,
        debug_reset_eth_chain,
        debug_set_btc_fee,
        debug_set_eth_account_nonce,
        debug_set_eth_gas_price,
        debug_set_key_in_db_to_value,
        get_enclave_state,
        get_latest_block_numbers,
        maybe_add_erc777_contract_address,
        maybe_initialize_btc_enclave,
        maybe_initialize_eth_enclave,
        sign_ascii_msg_with_eth_key_with_no_prefix,
        sign_hex_msg_with_eth_key_with_prefix,
        submit_btc_block_to_enclave,
        submit_eth_block_to_enclave,
    },
    databases::get_db,
    errors::AppError,
    loggers::init_logger,
};

use crate::{
    get_cli_args::{get_cli_args, CliArgs},
    get_versions::get_versions,
    usage_info::USAGE_INFO,
};

fn main() -> Result<(), AppError> {
    match init_logger()
        .and_then(|_| get_cli_args())
        .and_then(|cli_args| match cli_args {
            CliArgs {
                cmd_initializeEth: true,
                ..
            } => {
                info!("✔ Initializing ETH enclave...");
                Ok(maybe_initialize_eth_enclave(
                    get_db()?,
                    &cli_args.arg_blockJson,
                    cli_args.flag_chainId,
                    cli_args.flag_gasPrice,
                    cli_args.flag_confs,
                    "", // NOTE: Legacy bytecode path, now unused.
                )?)
            },
            CliArgs {
                cmd_initializeBtc: true,
                ..
            } => {
                info!("✔ Initializing BTC enclave...");
                Ok(maybe_initialize_btc_enclave(
                    get_db()?,
                    &cli_args.arg_blockJson,
                    cli_args.flag_fee,
                    cli_args.flag_difficulty,
                    &cli_args.flag_network,
                    cli_args.flag_confs,
                )?)
            },
            CliArgs {
                cmd_debugGetChildPaysForParentTx: true,
                ..
            } => {
                info!("✔ Debug getting `child-pays-for-parent` tx...");
                Ok(debug_get_child_pays_for_parent_btc_tx(
                    get_db()?,
                    cli_args.flag_fee,
                    &cli_args.arg_txId,
                    cli_args.arg_vOut,
                )?)
            },
            CliArgs {
                cmd_getEnclaveState: true,
                ..
            } => {
                info!("✔ Getting enclave state...");
                Ok(get_enclave_state(get_db()?)?)
            },
            CliArgs {
                cmd_debugGetAllDbKeys: true,
                ..
            } => {
                info!("✔ Debug getting all DB keys....");
                Ok(debug_get_all_db_keys()?)
            },
            CliArgs {
                cmd_getLatestBlockNumbers: true,
                ..
            } => {
                info!("✔ Maybe getting block numbers...");
                Ok(get_latest_block_numbers(get_db()?)?)
            },
            CliArgs {
                cmd_debugGetAllUtxos: true,
                ..
            } => {
                info!("✔ Debug getting all UTXOs from the database...");
                Ok(debug_get_all_utxos(get_db()?)?)
            },
            CliArgs {
                cmd_debugClearAllUtxos: true,
                ..
            } => {
                info!("✔ Debug clearing all UTXOs from the database...");
                Ok(debug_clear_all_utxos(&get_db()?)?)
            },
            CliArgs {
                cmd_debugGetKeyFromDb: true,
                ..
            } => {
                info!("✔ Maybe getting a key from the database...");
                Ok(debug_get_key_from_db(get_db()?, &cli_args.arg_key)?)
            },
            CliArgs {
                cmd_debugMaybeAddUtxoToDb: true,
                ..
            } => {
                info!("✔ Debug maybe adding UTXO to db...");
                Ok(debug_maybe_add_utxo_to_db(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugAddUtxos: true,
                ..
            } => {
                info!("✔ Debug adding multiple UTXOs...");
                Ok(debug_add_multiple_utxos(get_db()?, &cli_args.arg_utxosJson)?)
            },
            CliArgs {
                cmd_addErc777ContractAddress: true,
                ..
            } => {
                info!("✔ Maybe adding ERC777 contract address...");
                Ok(maybe_add_erc777_contract_address(get_db()?, &cli_args.arg_address)?)
            },
            CliArgs {
                cmd_debugReprocessEthBlock: true,
                ..
            } => {
                info!("✔ Debug reprocessing ETH block to enclave...");
                Ok(debug_reprocess_eth_block(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugReprocessBtcBlock: true,
                ..
            } => {
                info!("✔ Debug reprocessing BTC block to enclave...");
                Ok(debug_reprocess_btc_block(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_submitEthBlock: true,
                ..
            } => {
                info!("✔ Submitting ETH block to enclave...");
                Ok(submit_eth_block_to_enclave(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_submitBtcBlock: true,
                ..
            } => {
                info!("✔ Submitting BTC block to enclave...");
                Ok(submit_btc_block_to_enclave(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugRemoveUtxo: true,
                ..
            } => {
                info!("✔ Debug removing UTXO...");
                Ok(debug_remove_utxo(get_db()?, &cli_args.arg_txId, cli_args.arg_vOut)?)
            },
            CliArgs {
                cmd_signHexMsgWithEthKeyWithPrefix: true,
                ..
            } => {
                info!("✔ Signing HEX message with ETH key & ETH-specific prefix...");
                Ok(sign_hex_msg_with_eth_key_with_prefix(
                    &get_db()?,
                    &cli_args.arg_message,
                )?)
            },
            CliArgs {
                cmd_debugConsolidateUtxos: true,
                ..
            } => {
                info!("✔ Debug consolidating utxos...");
                Ok(debug_consolidate_utxos(
                    get_db()?,
                    cli_args.flag_fee,
                    cli_args.arg_numUtxos,
                )?)
            },
            CliArgs {
                cmd_debugErc777ChangePNetwork: true,
                ..
            } => {
                info!("✔ Debug getting `changePNetwork` tx...");
                Ok(debug_get_signed_erc777_change_pnetwork_tx(
                    get_db()?,
                    &cli_args.arg_address,
                )?)
            },
            CliArgs {
                cmd_signMessageWithEthKey: true,
                ..
            }
            | CliArgs {
                cmd_signAsciiMsgWithEthKeyWithNoPrefix: true,
                ..
            } => {
                info!("✔ Signing ASCII message with ETH key & NO prefix...");
                Ok(sign_ascii_msg_with_eth_key_with_no_prefix(
                    &get_db()?,
                    &cli_args.arg_message,
                )?)
            },
            CliArgs {
                cmd_debugSetKeyInDbToValue: true,
                ..
            } => {
                info!("✔ Setting a key in the database to a value...");
                Ok(debug_set_key_in_db_to_value(
                    get_db()?,
                    &cli_args.arg_key,
                    &cli_args.arg_value,
                )?)
            },
            CliArgs {
                cmd_debugErc777ProxyChangePNetwork: true,
                ..
            } => {
                info!("✔ Debug getting `changePNetwork` in the proxy tx...");
                Ok(debug_get_signed_erc777_proxy_change_pnetwork_tx(
                    get_db()?,
                    &cli_args.arg_address,
                )?)
            },
            CliArgs {
                cmd_debugWithdrawFees: true,
                ..
            } => {
                info!("✔ Debug withdrawing fees...");
                Ok(debug_get_fee_withdrawal_tx(get_db()?, &cli_args.arg_address)?)
            },
            CliArgs {
                cmd_debugErc777ProxyChangePNetworkByProxy: true,
                ..
            } => {
                info!("✔ Debug getting `changePNetworkByProxy` tx...");
                Ok(debug_get_signed_erc777_proxy_change_pnetwork_by_proxy_tx(
                    get_db()?,
                    &cli_args.arg_address,
                )?)
            },
            CliArgs {
                cmd_debugSetPegInFee: true,
                ..
            } => {
                info!(
                    "✔ Debug setting peg-in fee to {} basis points...",
                    cli_args.arg_basisPoints
                );
                Ok(debug_put_btc_on_eth_peg_in_basis_points_in_db(
                    &get_db()?,
                    cli_args.arg_basisPoints,
                )?)
            },
            CliArgs {
                cmd_debugSetEthGasPrice: true,
                ..
            } => {
                info!("✔ Debug setting ETH gas price to {} Wei..", cli_args.arg_wei);
                Ok(debug_set_eth_gas_price(get_db()?, cli_args.arg_wei)?)
            },
            CliArgs {
                cmd_debugSetPegOutFee: true,
                ..
            } => {
                info!(
                    "✔ Debug setting peg-out fee to {} basis points...",
                    cli_args.arg_basisPoints
                );
                Ok(debug_put_btc_on_eth_peg_out_basis_points_in_db(
                    &get_db()?,
                    cli_args.arg_basisPoints,
                )?)
            },
            CliArgs {
                cmd_debugSetBtcFee: true,
                ..
            } => {
                info!("✔ Debug setting BTC fee to {} Satoshis-per-byte...", cli_args.arg_fee);
                Ok(debug_set_btc_fee(get_db()?, cli_args.arg_fee)?)
            },
            CliArgs {
                cmd_debugResetEthChain: true,
                ..
            } => {
                info!("✔ Debug resetting ETH chain...");
                Ok(debug_reset_eth_chain(
                    get_db()?,
                    &cli_args.arg_blockJson,
                    cli_args.flag_confs,
                )?)
            },
            CliArgs {
                cmd_debugSetEthAccountNonce: true,
                ..
            } => {
                info!("✔ Debug setting ETH account nonce...");
                Ok(debug_set_eth_account_nonce(&get_db()?, cli_args.arg_nonce)?)
            },
            CliArgs { flag_version: true, .. } => get_versions(),
            _ => Err(AppError::Custom(USAGE_INFO.to_string())),
        }) {
        Ok(json_string) => {
            trace!("{}", json_string);
            println!("{}", json_string);
            Ok(())
        },
        Err(e) => {
            error!("{}", e);
            println!("{}", e);
            std::process::exit(1);
        },
    }
}
