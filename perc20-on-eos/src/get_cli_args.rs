use std::{fs::read_to_string, path::Path};

use docopt::Docopt;
use lib::types::Result;

use crate::usage_info::USAGE_INFO;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub flag_confs: u64,
    pub flag_file: String,
    pub flag_gasPrice: u64,
    pub flag_version: bool,
    pub flag_chainId: String,
    pub arg_wei: u64,
    pub arg_nonce: u64,
    pub arg_key: String,
    pub arg_value: String,
    pub arg_message: String,
    pub arg_eosJson: String,
    pub arg_blockJson: String,
    pub arg_entryJson: String,
    pub arg_ethAddress: String,
    pub arg_featureHash: String,
    pub arg_scheduleJson: String,
    pub cmd_initializeEos: bool,
    pub cmd_initializeEth: bool,
    pub cmd_submitEosBlock: bool,
    pub cmd_submitEthBlock: bool,
    pub cmd_getEnclaveState: bool,
    pub cmd_debugResetEthChain: bool,
    pub cmd_getLatestBlockNumbers: bool,
    pub cmd_addVaultContractAddress: bool,
    pub cmd_enableEosProtocolFeature: bool,
    pub cmd_disableEosProtocolFeature: bool,
    pub cmd_debugGetAllDbKeys: bool,
    pub cmd_debugGetKeyFromDb: bool,
    pub cmd_debugSetEthGasPrice: bool,
    pub cmd_debugAddEosSchedule: bool,
    pub cmd_debugMigrateContract: bool,
    pub cmd_debugReprocessEthBlock: bool,
    pub cmd_debugReprocessEosBlock: bool,
    pub cmd_debugSetKeyInDbToValue: bool,
    pub cmd_debugUpdateIncremerkle: bool,
    pub cmd_debugAddSupportedToken: bool,
    pub cmd_debugSetEthAccountNonce: bool,
    pub cmd_debugAddDictionaryEntry: bool,
    pub cmd_debugRemoveSupportedToken: bool,
    pub cmd_debugRemoveDictionaryEntry: bool,
    pub cmd_signMessageWithEthKey: bool,
    pub cmd_signHexMsgWithEthKeyWithPrefix: bool,
    pub cmd_signAsciiMsgWithEthKeyWithNoPrefix: bool,
}

impl CliArgs {
    pub fn update_block_in_cli_args(mut self, block_json: String) -> Result<Self> {
        self.arg_blockJson = block_json;
        Ok(self)
    }

    pub fn update_schedule_in_cli_args(mut self, json: String) -> Result<Self> {
        self.arg_scheduleJson = json;
        Ok(self)
    }

    pub fn update_eos_json_in_cli_args(mut self, json: String) -> Result<Self> {
        self.arg_eosJson = json;
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

pub fn maybe_read_json_from_file(cli_args: CliArgs) -> Result<CliArgs> {
    match Path::new(&cli_args.flag_file).exists() {
        true => {
            info!("✔ File exists @ path: {},\n✔ Reading file...", cli_args.flag_file);
            match cli_args {
                CliArgs {
                    cmd_initializeEos: true,
                    ..
                }
                | CliArgs {
                    cmd_debugUpdateIncremerkle: true,
                    ..
                } => {
                    info!("✔ Updating `eosJson` in CLI args...");
                    cli_args
                        .clone()
                        .update_eos_json_in_cli_args(read_to_string(cli_args.flag_file)?)
                },
                CliArgs {
                    cmd_debugAddEosSchedule: true,
                    ..
                } => {
                    info!("✔ Updating EOS schedule in CLI args...");
                    cli_args
                        .clone()
                        .update_schedule_in_cli_args(read_to_string(cli_args.flag_file)?)
                },
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
            info!("✔ No file exists @ path: {}\n✔ Not reading file...", cli_args.flag_file);
            Ok(cli_args)
        },
    }
}

pub fn get_cli_args() -> Result<CliArgs> {
    parse_cli_args().and_then(maybe_read_json_from_file)
}
