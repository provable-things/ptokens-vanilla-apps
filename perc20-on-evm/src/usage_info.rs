pub static USAGE_INFO: &str = "
❍ Provable Vanilla pERC20-on-EVM Enclave ❍

    Copyright Provable 2021
    Questions: greg@oraclize.it

❍ Info ❍

This Provable vanilla pERC20-on-EVM app uses the pToken core in order to manage the cross-chain conversions between
native ethereum ERC20 tokens and their EVM-compliant chain pTokenized counterparts.

❍ Usage ❍

Usage:  perc20-on-evm [--help]
        perc20-on-evm [--version]
        perc20-on-evm getEnclaveState
        perc20-on-evm getLatestBlockNumbers
        perc20-on-evm addVaultContractAddress <ethAddress>
        perc20-on-evm submitEthBlock (<blockJson> | --file=<path>)
        perc20-on-evm submitEvmBlock (<blockJson> | --file=<path>)
        perc20-on-evm initializeEvm (<blockJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        perc20-on-evm initializeEth (<blockJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        perc20-on-evm debugGetAllDbKeys
        perc20-on-evm debugGetKeyFromDb <key>
        perc20-on-evm debugSetEthGasPrice <gasPrice>
        perc20-on-evm debugSetEvmGasPrice <gasPrice>
        perc20-on-evm debugSetEthAccountNonce <nonce>
        perc20-on-evm debugSetEvmAccountNonce <nonce>
        perc20-on-evm debugMigrateContract <ethAddress>
        perc20-on-evm debugAddSupportedToken <ethAddress>
        perc20-on-evm debugSetKeyInDbToValue <key> <value>
        perc20-on-evm debugRemoveSupportedToken <ethAddress>
        perc20-on-evm debugRemoveDictionaryEntry <ethAddress>
        perc20-on-evm debugSetFeeBasisPoints <ethAddress> <fee>
        perc20-on-evm debugWithdrawFees <tokenAddress> <recipientAddress>
        perc20-on-evm debugReprocessEthBlock (<blockJson> | --file=<path>)
        perc20-on-evm debugReprocessEvmBlock (<blockJson> | --file=<path>)
        perc20-on-evm debugAddDictionaryEntry (<entryJson> | --file=<path>)
        perc20-on-evm debugResetEthChain (<blockJson> | --file=<path>) [--confs=<uint>]
        perc20-on-evm debugResetEvmChain (<blockJson> | --file=<path>) [--confs=<uint>]
        perc20-on-evm debugReprocessEthBlockWithFeeAccrual (<blockJson> | --file=<path>)
        perc20-on-evm debugReprocessEvmBlockWithFeeAccrual (<blockJson> | --file=<path>)
        perc20-on-evm signHexMsgWithEthKeyWithPrefix <message>
        perc20-on-evm signHexMsgWithEvmKeyWithPrefix <message>
        perc20-on-evm signAsciiMsgWithEthKeyWithNoPrefix <message>
        perc20-on-evm signAsciiMsgWithEvmKeyWithNoPrefix <message>

Commands:

    submitEthBlock                      ❍ Submit an ETH block (& its receipts) to the enclave.  NOTE: The enclave must
                                          first have been initialized!
                                          ➔ blockJson Format:
                                          {
                                            `block`: The block header itself.
                                            `receipts`: An array containing the block's receipts,
                                          }

    submitEvmBlock                      ❍ Submit an EVM block (& its receipts) to the enclave.  NOTE: The enclave must
                                          first have been initialized!
                                          ➔ blockJson Format:
                                          {
                                            `block`: The block header itself.
                                            `receipts`: An array containing the block's receipts,
                                          }

    initializeEth                       ❍ Initialize the enclave with the first trusted ETH block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignored by
                                          the enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its ETH
                                          related database from this trusted block, create the ETH private-key and seal
                                          it plus any relevant settings from the `config` into the database.
                                          This command will return a signed transaction to broadcast, which transaction
                                          will deploy the pToken contract to the ETH network.
                                          ➔ blockJson Format: See `submitEthBlock` for breakdown of JSON.

    initializeEvm                       ❍ Initialize the enclave with the first trusted EVM block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignored by
                                          the enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its ETH
                                          related database from this trusted block, create the ETH private-key and seal
                                          it plus any relevant settings from the `config` into the database.
                                          This command will return a signed transaction to broadcast, which transaction
                                          will deploy the pToken contract to the ETH network.
                                          ➔ blockJson Format: See `submitEvmBlock` for breakdown of JSON.

    getEnclaveState                     ❍ Returns the current state of the enclave as pulled from the database.

    addVaultContractAddress             ❍ Adds the ERC20 vault contract address to the encrypted database.

    debugGetAllDbKeys                   ❍ Returns JSON formatted report of all the database keys used in the core.

    debugWithdrawFees                   ❍ Withdraw fees for a given token address and send them to the given recipient
                                          address.
    debugSetEthGasPrice                 ❍ Sets the ETH gas price to use when making ETH transactions. (Unit: Wei)

    debugSetEvmGasPrice                 ❍ Sets the EVM gas price to use when making ETH transactions. (Unit: Wei)

    debugMigrateContract                ❍ Create a transaction that will migrate then current balances held by the
                                          `ERC20-vault` smart-contract to the `<ethAddress>` supplied.

    debugGetKeyFromDb                   ❍ Get a given <key> from the database. This function can only be called if the
                                          `debug` flag is set to true when the tool was built.

    debugResetEthChain                  ❍ Resets the ETH chain in the encrypted database using the supplied block as a
                                          new starting point.

    debugResetEvmChain                  ❍ Resets the EVM chain in the encrypted database using the supplied block as a
                                          new starting point.

    signHexMsgWithEthKeyWithPrefix      ❍ Signs an ASCII message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and NO prefix is
                                          prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signHexMsgWithEvmKeyWithPrefix      ❍ Signs an ASCII message with the EVM private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and NO prefix is
                                          prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signAsciiMsgWithEthKeyWithNoPrefix  ❍ Signs a HEX message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and the standard
                                          ethereum-specific prefix IS prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signAsciiMsgWithEvmKeyWithNoPrefix  ❍ Signs a HEX message with the EVM private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and the standard
                                          ethereum-specific prefix IS prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    getLatestBlockNumbers               ❍ Returns the current lastest ETH & BTC block numbers seen by the enclave.

    debugReprocessEthBlock              ❍ Submit ETH block submisson material for re-processing.

    debugReprocessEvmBlock              ❍ Submit EVM block submisson material for re-processing.

    debugReprocessEthBlockWithFeeAccrual ❍ Submit ETH block submisson material for re-processing, adding to accrued fees
                                           whilst doing so.

    debugReprocessEvmBlockWithFeeAccrual ❍ Submit EVM block submisson material for re-processing, adding to accrued fees
                                           whilst doing so.

    debugSetKeyInDbToValue              ❍ Set a given <key> in the database to a given <value>. This function can only
                                          be called if the `debug` flag is set to true when the core is built. Note that
                                          there are zero checks on what is passed in to the database: Use at own risk!

    debugAddDictionaryEntry             ❍ Add a dictionary entry.

    debugRemoveDictionaryEntry          ❍ Remove a dictionary entry via its ETH address.


    <key>                               ❍ A database key in HEX format.

    <value>                             ❍ A database value in HEX format.

    <blockJson>                         ❍ Valid JSON string of ETH or BTC block.

    <path>                              ❍ Path to the file being submitted to the app.

    <nonce>                             ❍ A nonce (as a 64 bit, unsigned integer).

    <message>                           ❍ A message to be signed.

    <gasPrice>                          ❍ The gas price (in Wei) to when making transactions.

    <ethAddress>                        ❍ A valid ethereum address in hex format.

    <tokenAddress>                      ❍ A valid ethereum token address.

    <recipientAddress>                  ❍ A valid ethereum fee recipient address.

    <fee>                               ❍ Fee value in basis points.

    <entryJson>                         ❍ Valid JSON string of a dictionary entry.
                                          ➔ JSON Format:
                                          {
                                             `eth_symbol`: The ETH token symbol,
                                             `evm_symbol`: The EVM token symbol,
                                             `eth_address`: The ETH token address,
                                             `evm_address`: The EVM token address,
                                          }

Options:

    --help                              ❍ Show this message.

    --version                           ❍ Returns the core version as well as the application version.

    --file=<path>                       ❍ Path to file containg an ETH or BTC block JSON.

    --gasPrice=<uint>                   ❍ The gas price to be used in ETH transactions.
                                          [default: 20000000000]

    --confs=<uint>                      ❍ The number of confirmations required before signing transactions. This affects
                                          the length of chain the light client maintains in the database.
                                          [default: 0]

    --chainId=<uint>                    ❍ ID of desired chain for transaction:
                                          1  = Ethereum Main-Net (default)
                                          3  = Ropsten Test-Net
                                          4  = Rinkeby Test-Net
                                          42 = Kovan Test-Net
                                          [default: 1]

    --ethNetwork=<str>                  ❍ Transaction network name
                                            - mainnet
                                            - ropsten
                                            - rinkeby
                                            - kovan

    --gasPrice=<uint>                   ❍ Transaction gas price
";
