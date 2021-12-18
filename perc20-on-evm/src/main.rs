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
    erc20_on_evm::{
        debug_add_dictionary_entry,
        debug_get_add_supported_token_tx,
        debug_get_all_db_keys,
        debug_get_erc20_on_evm_vault_migration_tx,
        debug_get_key_from_db,
        debug_get_remove_supported_token_tx,
        debug_remove_dictionary_entry,
        debug_reprocess_eth_block,
        debug_reprocess_eth_block_with_fee_accrual,
        debug_reprocess_evm_block,
        debug_reprocess_evm_block_with_fee_accrual,
        debug_reset_eth_chain,
        debug_reset_evm_chain,
        debug_set_eth_account_nonce,
        debug_set_eth_gas_price,
        debug_set_evm_account_nonce,
        debug_set_evm_gas_price,
        debug_set_fee_basis_points,
        debug_set_key_in_db_to_value,
        debug_withdraw_fees_and_save_in_db,
        get_enclave_state,
        get_latest_block_numbers,
        maybe_add_vault_contract_address,
        maybe_initialize_eth_core,
        maybe_initialize_evm_core,
        sign_ascii_msg_with_eth_key_with_no_prefix,
        sign_ascii_msg_with_evm_key_with_no_prefix,
        sign_hex_msg_with_eth_key_with_prefix,
        sign_hex_msg_with_evm_key_with_prefix,
        submit_eth_block_to_core,
        submit_evm_block_to_core,
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
                info!("✔ Initializing ETH core...");
                Ok(maybe_initialize_eth_core(
                    get_db()?,
                    &cli_args.arg_blockJson,
                    cli_args.flag_chainId,
                    cli_args.flag_gasPrice,
                    cli_args.flag_confs,
                )?)
            },
            CliArgs {
                cmd_initializeEvm: true,
                ..
            } => {
                info!("✔ Initializing EVM core...");
                Ok(maybe_initialize_evm_core(
                    get_db()?,
                    &cli_args.arg_blockJson,
                    cli_args.flag_chainId,
                    cli_args.flag_gasPrice,
                    cli_args.flag_confs,
                )?)
            },
            CliArgs {
                cmd_getEnclaveState: true,
                ..
            } => {
                info!("✔ Getting core state...");
                Ok(get_enclave_state(get_db()?)?)
            },
            CliArgs {
                cmd_debugMigrateContract: true,
                ..
            } => {
                info!("✔ Debug getting `migrate` transaction...");
                Ok(debug_get_erc20_on_evm_vault_migration_tx(
                    get_db()?,
                    &cli_args.arg_ethAddress,
                )?)
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
                cmd_debugReprocessEvmBlock: true,
                ..
            } => {
                info!("✔ Debug reprocessing EVM block...");
                Ok(debug_reprocess_evm_block(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_debugReprocessEthBlockWithFeeAccrual: true,
                ..
            } => {
                info!("✔ Debug reprocessing ETH block with fee accrual...");
                Ok(debug_reprocess_eth_block_with_fee_accrual(
                    get_db()?,
                    &cli_args.arg_blockJson,
                )?)
            },
            CliArgs {
                cmd_debugReprocessEvmBlockWithFeeAccrual: true,
                ..
            } => {
                info!("✔ Debug reprocessing EVM block with fee accrual...");
                Ok(debug_reprocess_evm_block_with_fee_accrual(
                    get_db()?,
                    &cli_args.arg_blockJson,
                )?)
            },
            CliArgs {
                cmd_submitEthBlock: true,
                ..
            } => {
                info!("✔ Submitting ETH block to core...");
                Ok(submit_eth_block_to_core(get_db()?, &cli_args.arg_blockJson)?)
            },
            CliArgs {
                cmd_submitEvmBlock: true,
                ..
            } => {
                info!("✔ Submitting EVM block to core...");
                Ok(submit_evm_block_to_core(get_db()?, &cli_args.arg_blockJson)?)
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
                cmd_signHexMsgWithEvmKeyWithPrefix: true,
                ..
            } => {
                info!("✔ Signing HEX message with EVM key & ETH-specific prefix...");
                Ok(sign_hex_msg_with_evm_key_with_prefix(
                    &get_db()?,
                    &cli_args.arg_message,
                )?)
            },
            CliArgs {
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
                cmd_signAsciiMsgWithEvmKeyWithNoPrefix: true,
                ..
            } => {
                info!("✔ Signing ASCII message with EVM key & NO prefix...");
                Ok(sign_ascii_msg_with_evm_key_with_no_prefix(
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
                cmd_debugAddDictionaryEntry: true,
                ..
            } => {
                info!("✔ Debug adding dictionary entry...");
                Ok(debug_add_dictionary_entry(get_db()?, &cli_args.arg_entryJson)?)
            },
            CliArgs {
                cmd_debugRemoveDictionaryEntry: true,
                ..
            } => {
                info!("✔ Debug removing dictionary entry...");
                Ok(debug_remove_dictionary_entry(get_db()?, &cli_args.arg_ethAddress)?)
            },
            CliArgs {
                cmd_debugAddSupportedToken: true,
                ..
            } => {
                info!("✔ Debug getting `addSupportedToken` signed transaction...");
                Ok(debug_get_add_supported_token_tx(get_db()?, &cli_args.arg_ethAddress)?)
            },
            CliArgs {
                cmd_debugRemoveSupportedToken: true,
                ..
            } => {
                info!("✔ Debug getting `removeSupportedToken` signed transaction...");
                Ok(debug_get_remove_supported_token_tx(
                    get_db()?,
                    &cli_args.arg_ethAddress,
                )?)
            },
            CliArgs {
                cmd_addVaultContractAddress: true,
                ..
            } => {
                info!("✔ Maybe adding vault contract address to DB...");
                Ok(maybe_add_vault_contract_address(get_db()?, &cli_args.arg_ethAddress)?)
            },
            CliArgs {
                cmd_debugSetFeeBasisPoints: true,
                ..
            } => {
                info!("✔ Debug setting fee basis points...");
                Ok(debug_set_fee_basis_points(
                    get_db()?,
                    &cli_args.arg_ethAddress,
                    cli_args.arg_fee,
                )?)
            },
            CliArgs {
                cmd_debugWithdrawFees: true,
                ..
            } => {
                info!("✔ Debug withdrawing fees...");
                Ok(debug_withdraw_fees_and_save_in_db(
                    get_db()?,
                    &cli_args.arg_tokenAddress,
                    &cli_args.arg_recipientAddress,
                )?)
            },
            CliArgs {
                cmd_debugSetEthGasPrice: true,
                ..
            } => {
                info!("✔ Debug setting ETH gas price...");
                Ok(debug_set_eth_gas_price(get_db()?, cli_args.arg_gasPrice)?)
            },
            CliArgs {
                cmd_debugSetEvmGasPrice: true,
                ..
            } => {
                info!("✔ Debug setting EVM gas price...");
                Ok(debug_set_evm_gas_price(get_db()?, cli_args.arg_gasPrice)?)
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
                cmd_debugResetEvmChain: true,
                ..
            } => {
                info!("✔ Debug resetting EVM chain...");
                Ok(debug_reset_evm_chain(
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
            CliArgs {
                cmd_debugSetEvmAccountNonce: true,
                ..
            } => {
                info!("✔ Debug setting EVM account nonce...");
                Ok(debug_set_evm_account_nonce(&get_db()?, cli_args.arg_nonce)?)
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
