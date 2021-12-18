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
    databases::get_db,
    eos_on_eth::{
        debug_add_eos_eth_token_dictionary_entry,
        debug_add_new_eos_schedule,
        debug_get_all_db_keys,
        debug_get_key_from_db,
        debug_remove_eos_eth_token_dictionary_entry,
        debug_reprocess_eos_block,
        debug_reprocess_eth_block,
        debug_reset_eth_chain,
        debug_set_eth_account_nonce,
        debug_set_eth_gas_price,
        debug_set_key_in_db_to_value,
        debug_update_incremerkle,
        disable_eos_protocol_feature,
        enable_eos_protocol_feature,
        get_enclave_state,
        get_latest_block_numbers,
        maybe_initialize_eos_core,
        maybe_initialize_eth_core,
        sign_ascii_msg_with_eth_key_with_no_prefix,
        sign_hex_msg_with_eth_key_with_prefix,
        submit_eos_block_to_core,
        submit_eth_block_to_core,
    },
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
                let chain_id = cli_args.flag_chainId.parse()?;
                Ok(maybe_initialize_eth_core(
                    get_db()?,
                    &cli_args.arg_blockJson,
                    chain_id,
                    cli_args.flag_gasPrice,
                    cli_args.flag_confs,
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
                cmd_debugGetKeyFromDb: true,
                ..
            } => {
                info!("✔ Maybe getting a key from the database...");
                Ok(debug_get_key_from_db(get_db()?, &cli_args.arg_key)?)
            },
            CliArgs {
                cmd_debugReprocessEthBlock: true,
                ..
            } => {
                info!("✔ Debug reprocessing ETH block...");
                Ok(debug_reprocess_eth_block(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_submitEthBlock: true,
                ..
            } => {
                info!("✔ Submitting ETH block to core...");
                Ok(submit_eth_block_to_core(get_db()?, &cli_args.arg_blockJson)?)
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
                cmd_enableEosProtocolFeature: true,
                ..
            } => {
                info!("✔ Enabled EOS protocol feature...");
                Ok(enable_eos_protocol_feature(get_db()?, &cli_args.arg_featureHash)?)
            },
            CliArgs {
                cmd_initializeEos: true,
                ..
            } => {
                info!("✔ Maybe initializing EOS core...");
                Ok(maybe_initialize_eos_core(
                    get_db()?,
                    &cli_args.flag_chainId,
                    &cli_args.flag_accountName,
                    &cli_args.arg_eosJson,
                )?)
            },
            CliArgs {
                cmd_disableEosProtocolFeature: true,
                ..
            } => {
                info!("✔ Disabling EOS protocol feature...");
                Ok(disable_eos_protocol_feature(get_db()?, &cli_args.arg_featureHash)?)
            },
            CliArgs {
                cmd_debugAddEosSchedule: true,
                ..
            } => {
                info!("✔ Adding EOS schedule to database...");
                Ok(debug_add_new_eos_schedule(get_db()?, &cli_args.arg_scheduleJson)?)
            },
            CliArgs {
                cmd_submitEosBlock: true,
                ..
            } => {
                info!("✔ Submitting EOS block to core...");
                Ok(submit_eos_block_to_core(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugRemoveDictionaryEntry: true,
                ..
            } => {
                info!("✔ Debug removing `EosEthDictionary` entry...");
                Ok(debug_remove_eos_eth_token_dictionary_entry(
                    get_db()?,
                    &cli_args.arg_ethAddress,
                )?)
            },
            CliArgs {
                cmd_debugUpdateIncremerkle: true,
                ..
            } => {
                info!("✔ Debug updating EOS incremerkle...");
                Ok(debug_update_incremerkle(&get_db()?, &cli_args.arg_eosJson)?)
            },
            CliArgs {
                cmd_debugReprocessEosBlock: true,
                ..
            } => {
                info!("✔ Debug reprocess EOS block...");
                Ok(debug_reprocess_eos_block(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugAddDictionaryEntry: true,
                ..
            } => {
                info!("✔ Debug adding `EosEthDictionary` entry...");
                Ok(debug_add_eos_eth_token_dictionary_entry(
                    get_db()?,
                    &cli_args.arg_entryJson,
                )?)
            },
            CliArgs {
                cmd_debugSetEthGasPrice: true,
                ..
            } => {
                info!("✔ Debug setting ETH gas price to {} wei...", cli_args.arg_wei);
                Ok(debug_set_eth_gas_price(get_db()?, cli_args.arg_wei)?)
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
