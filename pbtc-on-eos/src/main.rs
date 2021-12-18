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
    btc_on_eos::{
        debug_add_multiple_utxos,
        debug_add_new_eos_schedule,
        debug_consolidate_utxos,
        debug_get_all_db_keys,
        debug_get_all_utxos,
        debug_get_child_pays_for_parent_btc_tx,
        debug_get_key_from_db,
        debug_maybe_add_utxo_to_db,
        debug_remove_utxo,
        debug_reprocess_btc_block_for_stale_eos_tx,
        debug_set_btc_fee,
        debug_set_key_in_db_to_value,
        debug_update_incremerkle,
        disable_eos_protocol_feature,
        enable_eos_protocol_feature,
        get_enclave_state,
        get_latest_block_numbers,
        maybe_initialize_btc_core,
        maybe_initialize_eos_core,
        submit_btc_block_to_core,
        submit_eos_block_to_core,
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
                cmd_initializeEos: true,
                ..
            } => {
                info!("✔ Maybe initializing EOS core...");
                Ok(maybe_initialize_eos_core(
                    get_db()?,
                    &cli_args.flag_chainId,
                    &cli_args.arg_accountName,
                    &cli_args.flag_symbol,
                    &cli_args.arg_eosJson,
                )?)
            },
            CliArgs {
                cmd_initializeBtc: true,
                ..
            } => {
                info!("✔ Initializing BTC core...");
                Ok(maybe_initialize_btc_core(
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
                cmd_debugGetAllDbKeys: true,
                ..
            } => {
                info!("✔ Debug getting all DB keys...");
                Ok(debug_get_all_db_keys()?)
            },
            CliArgs {
                cmd_getEnclaveState: true,
                ..
            } => {
                info!("✔ Getting core state...");
                Ok(get_enclave_state(get_db()?)?)
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
                info!("✔ Getting all UTXOs from the database...");
                Ok(debug_get_all_utxos(get_db()?)?)
            },
            CliArgs {
                cmd_debugGetKeyFromDb: true,
                ..
            } => {
                info!("✔ Maybe getting a key from the database...");
                Ok(debug_get_key_from_db(get_db()?, &cli_args.arg_key)?)
            },
            CliArgs {
                cmd_debugUpdateIncremerkle: true,
                ..
            } => {
                info!("✔ Debug updating EOS incremerkle...");
                Ok(debug_update_incremerkle(&get_db()?, &cli_args.arg_eosJson)?)
            },
            CliArgs {
                cmd_submitEosBlock: true,
                ..
            } => {
                info!("✔ Submitting EOS block to core...");
                Ok(submit_eos_block_to_core(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugAddUtxos: true,
                ..
            } => {
                info!("✔ Debug adding multiple UTXOs...");
                Ok(debug_add_multiple_utxos(get_db()?, &cli_args.arg_utxosJson)?)
            },
            CliArgs {
                cmd_submitBtcBlock: true,
                ..
            } => {
                info!("✔ Submitting BTC block to core...");
                Ok(submit_btc_block_to_core(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugAddEosSchedule: true,
                ..
            } => {
                info!("✔ Adding EOS schedule to database...");
                Ok(debug_add_new_eos_schedule(get_db()?, &cli_args.arg_scheduleJson)?)
            },
            CliArgs {
                cmd_enableEosProtocolFeature: true,
                ..
            } => {
                info!("✔ Enabled EOS protocol feature...");
                Ok(enable_eos_protocol_feature(get_db()?, &cli_args.arg_featureHash)?)
            },
            CliArgs {
                cmd_disableEosProtocolFeature: true,
                ..
            } => {
                info!("✔ Disabling EOS protocol feature...");
                Ok(disable_eos_protocol_feature(get_db()?, &cli_args.arg_featureHash)?)
            },
            CliArgs {
                cmd_debugRemoveUtxo: true,
                ..
            } => {
                info!("✔ Debug removing UTXO...");
                Ok(debug_remove_utxo(get_db()?, &cli_args.arg_txId, cli_args.arg_vOut)?)
            },
            CliArgs {
                cmd_debugConsolidateUtxos: true,
                ..
            } => {
                info!("✔ Debug consolidating UTXOS...");
                Ok(debug_consolidate_utxos(
                    get_db()?,
                    cli_args.flag_fee,
                    cli_args.arg_numUtxos,
                )?)
            },
            CliArgs {
                cmd_debugReprocessBtcBlock: true,
                ..
            } => {
                info!("✔ Debug reprocessing BTC block...");
                Ok(debug_reprocess_btc_block_for_stale_eos_tx(
                    get_db()?,
                    &cli_args.arg_blockJson,
                )?)
            },
            CliArgs {
                cmd_debugMaybeAddUtxoToDb: true,
                ..
            } => {
                info!("✔ Debug maybe adding UTXO to db...");
                Ok(debug_maybe_add_utxo_to_db(get_db()?, &cli_args.arg_blockJson)?)
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
                cmd_debugSetBtcFee: true,
                ..
            } => {
                info!("✔ Debug setting BTC fee to {} Satoshis-per-byte...", cli_args.arg_fee);
                Ok(debug_set_btc_fee(get_db()?, cli_args.arg_fee)?)
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
