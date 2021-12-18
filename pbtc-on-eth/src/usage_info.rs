pub static USAGE_INFO: &str = "
❍ Provable Vanilla pBTC-on-ETH App ❍

    Copyright Provable 2021
    Questions: greg@oraclize.it

❍ Info ❍

This Provable vanilla pBTC-on-ETH app uses the pToken core in order to manage the cross-chain conversions between pBTC & BTC
tokens.

❍ Usage ❍

Usage:  pbtc-on-eth [--help]
        pbtc-on-eth [--version]
        pbtc-on-eth getEnclaveState
        pbtc-on-eth getLatestBlockNumbers
        pbtc-on-eth addErc777ContractAddress <address>
        pbtc-on-eth submitEthBlock (<blockJson> | --file=<path>)
        pbtc-on-eth submitBtcBlock (<blockJson> | --file=<path>)
        pbtc-on-eth initializeEth (<blocksJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        pbtc-on-eth initializeBtc (<blocksJson> | --file=<path>) [--network=<string>] [--difficulty=<uint>] [--fee=<uint>] [--confs=<uint>]
        pbtc-on-eth debugGetAllUtxos
        pbtc-on-eth debugGetAllDbKeys
        pbtc-on-eth debugClearAllUtxos
        pbtc-on-eth debugSetBtcFee <fee>
        pbtc-on-eth debugGetKeyFromDb <key>
        pbtc-on-eth debugSetEthGasPrice <wei>
        pbtc-on-eth debugWithdrawFees <address>
        pbtc-on-eth debugRemoveUtxo <txId> <vOut>
        pbtc-on-eth debugSetPegInFee <basisPoints>
        pbtc-on-eth debugSetPegOutFee <basisPoints>
        pbtc-on-eth debugSetEthAccountNonce <nonce>
        pbtc-on-eth debugErc777ChangePNetwork <address>
        pbtc-on-eth debugSetKeyInDbToValue <key> <value>
        pbtc-on-eth debugErc777ProxyChangePNetwork <address>
        pbtc-on-eth debugAddUtxos (<utxosJson> | --file=<path>)
        pbtc-on-eth debugConsolidateUtxos <numUtxos> [--fee=<uint>]
        pbtc-on-eth debugErc777ProxyChangePNetworkByProxy <address>
        pbtc-on-eth debugMaybeAddUtxoToDb (<blockJson> | --file=<path>)
        pbtc-on-eth debugReprocessBtcBlock (<blockJson> | --file=<path>)
        pbtc-on-eth debugReprocessEthBlock (<blockJson> | --file=<path>)
        pbtc-on-eth debugGetChildPaysForParentTx <txId> <vOut> [--fee=<uint>]
        pbtc-on-eth debugResetEthChain (<blockJson> | --file=<path>) [--confs=<uint>]
        pbtc-on-eth signMessageWithEthKey <message>
        pbtc-on-eth signHexMsgWithEthKeyWithPrefix <message>
        pbtc-on-eth signAsciiMsgWithEthKeyWithNoPrefix <message>

Commands:

    submitEthBlock                      ❍ Submit an ETH block (& its receipts) to the enclave.  NOTE: The enclave must
                                          first have been initialized!
                                          ➔ blockJson Format:
                                          {
                                            `Block`: The block header itself.
                                            `Receipt`: An array containing the block's receipts,
                                          }

    submitBtcBlock                      ❍ Submit an BTC block & its transactions to the enclave. The submission material
                                          must also include an array of deposit information for `p2sh` addresses. NOTE:
                                          The enclave must first have been initialized!
                                          ➔ blockJson Format:
                                          {
                                            `block`: The BTC block in JSON format.
                                            `transactions`: The transactions in HEX format.
                                            `deposit_address_list`: [
                                                {
                                                  `nonce`: An integer nonce.
                                                  `eth_address`: The destination ETH address in hex.
                                                  `btc_deposit_address`: The `p2sh` BTC deposit address.
                                                  `eth_address_and_nonce_hash`: The `sha256d` of `eth_address + nonce`
                                                },
                                            ]
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

    initializeBtc                       ❍ Initialize the enclave with the first trusted BTC block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignored by
                                          the enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its BTC
                                          related database from this trusted block, create the BTC private-key and seal
                                          it plus any relevant settings from the `config` into the database.
                                          ➔ blocksJson Format: See `submitBTCBlock` for breakdown of JSON.

    getEnclaveState                     ❍ Returns the current state of the enclave as pulled from the database.

    addErc777ContractAddress            ❍ Adds the ERC777 contract address for the pTokenized version of BTC.

    debugChangePnetwork                 ❍ Make the core output a tx which when broadcast will change the pNetwork
                                          address in the ERC777 contract.

    debugGetAllDbKeys                   ❍ Returns JSON formatted report of all the database keys used in the core.

    debugGetAllUtxos                    ❍ Returns JSON formatted report of all the UTXOs currently held in the DB. This
                                          function can only be called if the `debug` flag is set.

    debugSetEthGasPrice                 ❍ Set the gas price for ETH transactions.

    debugGetKeyFromDb                   ❍ Get a given <key> from the database. This function can only be called if the
                                          `debug` flag is set to true when the tool was built.

    debugSetBtcFee                      ❍ Sets the BTC fee to use when making transactions.


    signHexMsgWithEthKeyWithPrefix      ❍ Signs an ASCII message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and NO prefix is
                                          prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signAsciiMsgWithEthKeyWithNoPrefix  ❍ Signs a HEX message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and the standard
                                          ethereum-specific prefix IS prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signMessageWithEthKey               ❍ DEPRECATED! This is an alias for `signAsciiMsgWithEthKeyWithNoPrefix`

    debugClearAllUtxos                  ❍ Clear all the UTXOs set stored inside the database

    getLatestBlockNumbers               ❍ Returns the current lastest ETH & BTC block numbers seen by the enclave.

    debugMaybeAddUtxoToDb               ❍ Reprocess a BTC block looking for any UTXOs to add to the core.

    debugReprocessBtcBlock              ❍ Submit BTC block submisson material for re-processing.

    debugReprocessEthBlock              ❍ Submit ETH block submisson material for re-processing.

    debugResetEthChain                  ❍ Resets the ETH chain in the encrypted database using the supplied block as a
                                          new starting point.

    debugWithdrawFees                   ❍ Creates a BTC transaction to the passed in address for the amount of the total
                                          accrued fees in the core.

    debugSetKeyInDbToValue              ❍ Set a given <key> in the database to a given <value>. This function can only
                                          be called if the `debug` flag is set to true when the core is built. Note that
                                          there are zero checks on what is passed in to the database: Use at own risk!

    debugAddUtxos                       ❍ Adds multiple UTXOs to the core, if they are not already extant. Format of the
                                          JSON is the same as is outputted from the `debugGetAllUtxos` function.

    <key>                               ❍ A database key in HEX format.

    <wei>                              ❍ The ETH gas price in Wei.

    <value>                             ❍ A database value in HEX format.

    <address>                           ❍ A valid Ethereum  or Bitcoin address.

    <blockJson>                         ❍ Valid JSON string of ETH or BTC block.

    <txId>                              ❍ The transaction ID of a BTC UTXO.

    <vOut>                              ❍ The output of a BTC UTXO to spend.

    <numUtxos>                          ❍ The number of UTXOS to attempt to consolidate.

    <utxosJson>                         ❍ Valid JSON string of UTXOs per the format `debugGetAllUtxos` returns.

    <message>                           ❍ A message to be signed.

    <nonce>                             ❍ A nonce (as a 64 bit, unsigned integer).

    <basisPoints>                       ❍ Fee amount in basis points.

    <fee>                               ❍ The BTC transaction fee, in Satoshis-per-byte.

Options:

    --help                              ❍ Show this message.

    --version                           ❍ Returns the core version as well as the application version.

    --file=<path>                       ❍ Path to file containg an ETH or BTC block JSON.

    --fee=<uint>                        ❍ BTC fee as measured in Satoshis per byte.
                                          [default: 23]

    --difficulty=<path>                 ❍ The `difficulty` value above which a BTC block's difficulty should be in order
                                          to be considered valid.
                                          [default: 1337]

    --gasPrice=<uint>                   ❍ The gas price to be used in ETH transactions.
                                          [default: 20000000000]

    --confs=<uint>                      ❍ The number of confirmations required before signing transactions. This affects
                                          the length of chain the light client maintains in the database.
                                          [default: 0]

    --network=<string>                  ❍ Desired BTC network. Use `Bitcoin` for the maine bitcoin network, and use
                                          `Testnet` for the bitcoin public test-net
                                          [default: Bitcoin]

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

    --recipient=<str>                   ❍ Transaction eth address
";
