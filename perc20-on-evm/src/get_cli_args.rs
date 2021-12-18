use std::{fs::read_to_string, path::Path};

use docopt::Docopt;
use lib::types::Result;

use crate::usage_info::USAGE_INFO;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub flag_confs: u64,
    pub flag_chainId: u8, // NOTE: ETH network!
    pub flag_file: String,
    pub flag_gasPrice: u64,
    pub flag_version: bool,
    pub flag_ethNetwork: String,
    pub arg_fee: u64,
    pub arg_nonce: u64,
    pub arg_key: String,
    pub arg_value: String,
    pub arg_gasPrice: u64,
    pub arg_message: String,
    pub arg_entryJson: String,
    pub arg_blockJson: String,
    pub arg_ethAddress: String,
    pub arg_tokenAddress: String,
    pub arg_recipientAddress: String,
    pub cmd_initializeEth: bool,
    pub cmd_initializeEvm: bool,
    pub cmd_submitEthBlock: bool,
    pub cmd_submitEvmBlock: bool,
    pub cmd_getEnclaveState: bool,
    pub cmd_debugResetEthChain: bool,
    pub cmd_debugResetEvmChain: bool,
    pub cmd_getLatestBlockNumbers: bool,
    pub cmd_debugSetFeeBasisPoints: bool,
    pub cmd_addVaultContractAddress: bool,
    pub cmd_debugGetAllDbKeys: bool,
    pub cmd_debugGetKeyFromDb: bool,
    pub cmd_debugWithdrawFees: bool,
    pub cmd_debugSetEthGasPrice: bool,
    pub cmd_debugSetEvmGasPrice: bool,
    pub cmd_debugMigrateContract: bool,
    pub cmd_debugReprocessEthBlock: bool,
    pub cmd_debugReprocessEvmBlock: bool,
    pub cmd_debugSetKeyInDbToValue: bool,
    pub cmd_debugAddSupportedToken: bool,
    pub cmd_debugSetEthAccountNonce: bool,
    pub cmd_debugSetEvmAccountNonce: bool,
    pub cmd_debugAddDictionaryEntry: bool,
    pub cmd_debugRemoveSupportedToken: bool,
    pub cmd_debugRemoveDictionaryEntry: bool,
    pub cmd_signHexMsgWithEthKeyWithPrefix: bool,
    pub cmd_signHexMsgWithEvmKeyWithPrefix: bool,
    pub cmd_signAsciiMsgWithEthKeyWithNoPrefix: bool,
    pub cmd_signAsciiMsgWithEvmKeyWithNoPrefix: bool,
    pub cmd_debugReprocessEthBlockWithFeeAccrual: bool,
    pub cmd_debugReprocessEvmBlockWithFeeAccrual: bool,
}

impl CliArgs {
    pub fn update_block_in_cli_args(mut self, block_json: String) -> Result<Self> {
        self.arg_blockJson = block_json;
        Ok(self)
    }

    pub fn update_entry_json_in_cli_args(mut self, json: String) -> Result<Self> {
        self.arg_entryJson = json;
        Ok(self)
    }
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => Ok(cli_args),
        Err(e) => Err(e.into()),
    }
}

pub fn maybe_read_block_json_from_file(cli_args: CliArgs) -> Result<CliArgs> {
    match Path::new(&cli_args.flag_file).exists() {
        true => {
            info!("✔ File exists @ path: {}, reading file...", cli_args.flag_file);
            info!("✔ Updating block in CLI args...");
            match cli_args {
                CliArgs {
                    cmd_debugAddDictionaryEntry: true,
                    ..
                } => {
                    info!("✔ Updating entry json in CLI args...");
                    cli_args
                        .clone()
                        .update_entry_json_in_cli_args(read_to_string(cli_args.flag_file)?)
                },
                _ => {
                    info!("✔ Updating block in CLI args...");
                    cli_args
                        .clone()
                        .update_block_in_cli_args(read_to_string(cli_args.flag_file)?)
                },
            }
        },
        false => {
            info!("✔ No file exists @ path: {}, not reading file...", cli_args.flag_file);
            Ok(cli_args)
        },
    }
}

pub fn get_cli_args() -> Result<CliArgs> {
    parse_cli_args().and_then(maybe_read_block_json_from_file)
}
