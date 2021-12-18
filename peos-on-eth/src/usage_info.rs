pub static USAGE_INFO: &str = "
❍ Provable Vanilla pEOS-onETH App ❍

    Copyright Provable 2021
    Questions: greg@oraclize.it

❍ Info ❍

This Provable vanilla pEOS-on-ETH app uses the pToken core in order to manage the cross-chain conversions between native
EOS tokens and their pTokenized ethereum counterparts.

❍ Usage ❍

Usage:  peos-on-eth [--help]
        peos-on-eth [--version]
        peos-on-eth getEnclaveState
        peos-on-eth getLatestBlockNumbers
        peos-on-eth enableEosProtocolFeature <featureHash>
        peos-on-eth disableEosProtocolFeature <featureHash>
        peos-on-eth submitEthBlock (<blockJson> | --file=<path>)
        peos-on-eth submitEosBlock (<blockJson> | --file=<path>)
        peos-on-eth initializeEos [--accountName=<string>] [--chainId=<hex>] (<eosJson> | --file=<path>)
        peos-on-eth initializeEth (<blocksJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        peos-on-eth debugGetAllDbKeys
        peos-on-eth debugGetKeyFromDb <key>
        peos-on-eth debugSetEthGasPrice <wei>
        peos-on-eth debugSetEthAccountNonce <nonce>
        peos-on-eth debugSetKeyInDbToValue <key> <value>
        peos-on-eth debugRemoveDictionaryEntry <ethAddress>
        peos-on-eth debugUpdateIncremerkle (<eosJson> | --file=<path>)
        peos-on-eth debugReprocessEosBlock (<blockJson> | --file=<path>)
        peos-on-eth debugReprocessEthBlock (<blockJson> | --file=<path>)
        peos-on-eth debugReprocessEosBlock (<blockJson> | --file=<path>)
        peos-on-eth debugAddEosSchedule (<scheduleJson> | --file=<path>)
        peos-on-eth debugAddDictionaryEntry (<entryJson> | --file=<path>)
        peos-on-eth debugResetEthChain (<blockJson> | --file=<path>) [--confs=<uint>]
        peos-on-eth signMessageWithEthKey <message>
        peos-on-eth signHexMsgWithEthKeyWithPrefix <message>
        peos-on-eth signAsciiMsgWithEthKeyWithNoPrefix <message>

Commands:

    submitEthBlock                      ❍ Submit an ETH block (& its receipts) to the enclave.  NOTE: The enclave must
                                          first have been initialized!
                                          ➔ blockJson Format:
                                          {
                                            `Block`: The block header itself.
                                            `Receipt`: An array containing the block's receipts,
                                          }

    submitEosBlock                      ❍ Submit an EOS block (& its receipts) to the enclave.
                                          ➔ blockJson Format:
                                          {
                                              `block_header`: An EOS block header,
                                              `action_proofs`: An array of EOS action proofs,
                                              `interim_block_ids`: An array of EOS block IDs from the core's latest to
                                                                   the block above,
                                          }

    initializeEth                       ❍ Initialize the enclave with the first trusted ETH block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignored by
                                          the enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its ETH
                                          related database from this trusted block, create the ETH private-key and seal
                                          it plus any relevant settings from the `config` into the database.
                                          This command will return a signed transaction to broadcast, which transaction
                                          will deploy the pToken contract to the ETH network.
                                          ➔ blocksJson Format: See `submitETHBlock` for breakdown of JSON.

    initializeEos                       ❍ Initialize the enclave with the first trusted EOS block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignore by the
                                          enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its EOS
                                          related database from this trusted block, create the EOS private-key and seal
                                          it plus any relevant settings from the `config` into the database. This
                                          command will return a signed transaction to broadcast, which transaction will
                                          deploy the pToken contract to the EOS network.
                                          ➔ blockJson Format:

                                              `block`: An EOS block,
                                              `active_schedule`: The active schedule for the above block,
                                              `blockroot_merkle`: The blockroot-merkles for the above block,
                                              `erc20_on_eos_token_dictionary`: [{
                                                `eos_symbol`: Symbol for the EOS token,
                                                `eth_symbol`: Symbol for the ETH token,
                                                `eos_address`: Address of the EOS token,
                                                `eth_address`: Address of the ETH token,
                                                `eth_token_decimals`: Number of decimals in the ETH token,
                                                `eos_token_decimals`: Number of decimals in the EOS token,
                                              }]
                                          }

    getEnclaveState                     ❍ Returns the current state of the enclave as pulled from the database.

    debugGetAllDbKeys                   ❍ Returns JSON formatted report of all the database keys used in the core.

    debugSetEthGasPrice                 ❍ Set the gas price for ETH transactions.

    debugGetKeyFromDb                   ❍ Get a given <key> from the database. This function can only be called if the
                                          `debug` flag is set to true when the tool was built.


    signHexMsgWithEthKeyWithPrefix      ❍ Signs an ASCII message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and NO prefix is
                                          prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signAsciiMsgWithEthKeyWithNoPrefix  ❍ Signs a HEX message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and the standard
                                          ethereum-specific prefix IS prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signMessageWithEthKey               ❍ DEPRECATED! This is an alias for `signAsciiMsgWithEthKeyWithNoPrefix`

    getLatestBlockNumbers               ❍ Returns the current lastest ETH & BTC block numbers seen by the enclave.

    debugReprocessEosBlock              ❍ Submit BTC block submisson material for re-processing.

    debugReprocessEthBlock              ❍ Submit ETH block submisson material for re-processing.

    debugSetKeyInDbToValue              ❍ Set a given <key> in the database to a given <value>. This function can only
                                          be called if the `debug` flag is set to true when the core is built. Note that
                                          there are zero checks on what is passed in to the database: Use at own risk!

    enableEosProtocolFeature            ❍ Enable an EOS protocol feature in the core.

    disableEosProtocolFeature           ❍ Disable an EOS protocol feature in the core.

    debugAddEosSchedule                 ❍ Add an EOS schedule to the database.

    debugResetEthChain                  ❍ Resets the ETH chain in the encrypted database using the supplied block as a
                                          new starting point.

    debugUpdateIncremerkle              ❍ Use a trusted block header, blockroot_merkle and blockroot_merkle to update
                                          the EOS incremerkle in the database, thus effectively moving the chain forward
                                          to the submittied block's height.
                                          ➔ eosJson Format:
                                            {
                                              `block`: An EOS block,
                                              `active_schedule`: The active schedule for the above block,
                                              `blockroot_merkle`: The blockroot-merkles for the above block,
                                            }

    debugAddDictionaryEntry             ❍ Add an `EosErc20DictionaryEntry` to the core's encrypted databsae.

    debugRemoveDictionaryEntry          ❍ Remove an `EosErc20DictionaryEntry` to the core's encrypted databsae.

    debugReprocessEosBlock              ❍ Reprocess an EOS block by resubmitting it to the core.


    <key>                               ❍ A database key in HEX format.

    <value>                             ❍ A database value in HEX format.

    <blockJson>                         ❍ Valid JSON string of ETH or BTC block.

    <ethAddress>                        ❍ A valid ethereum address in hex format.

    <path>                              ❍ Path to file containing data relevnt to the chosen command.

    <wei>                               ❍ The ETH gas price in Wei.

    <nonce>                             ❍ A nonce (as a 64 bit, unsigned integer).

    <message>                           ❍ A message to be signed.

    <blockJson>                         ❍ Valid JSON string of EOS or ETH block.

    <scheduleJson>                      ❍ A valid EOS schedule JSON.

    <entryJson>                         ❍ Valid JSON string of a dictionary entry.
                                          ➔ JSON Format:
                                          {
                                             `eos_symbol`: The EOS token symbol,
                                             `eth_symbol`: The ETH token symbol,
                                             `eos_address`: The EOS token address,
                                             `eth_address`: The ETH token address,
                                             `eth_token_decimals`: The number of decimals the ETH token has,
                                             `eos_token_decimals`: The number of decimals the EOS token has,
                                          }

    <featureHash>                       ❍ A hash as a hex string of an EOS protocol feature.

    <eosJson>                           ❍ Valid JSON string of an object with the fields:
                                          ➔ JSON Format:
                                          {
                                            `block`: An EOS block,
                                            `active_schedule`: The active schedule for the above block,
                                            `blockroot_merkle`: The blockroot-merkles for the above block,
                                          }

Options:

    --help                              ❍ Show this message.

    --version                           ❍ Returns the core version as well as the application version.

    --file=<path>                       ❍ Path to file containing a JSON relevant to the chosen command.

    --gasPrice=<uint>                   ❍ The gas price to be used in ETH transactions.
                                          [default: 20000000000]

    --chainId=<hex|uint>                ❍ Hex string of the EOS chain ID, or integer for ETH chain ID:
                                            1 = Ethereum Main-Net (default)
                                            3 = Ropsten Test-Net
                                            4 = Rinkeby Test-Net
                                            42 = Kovan Test-Net
                                          [default: 1]

    --confs=<uint>                      ❍ The number of confirmations required before signing transactions. This affects
                                          the length of chain the light client maintains in the database.
                                          [default: 0]

    --accountName=<string>              ❍ Account name of the authorized user of the EOS smart contract.
                                          [default: pbtctokenxxx]

    --ethNetwork=<str>                  ❍ Transaction network name
                                            - mainnet
                                            - ropsten
                                            - rinkeby
                                            - kovan
";
