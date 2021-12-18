# :closed_lock_with_key: pERC20-on-EOS App

This Provable __pERC20-on-EOS__ app leverages the __pToken__ core in order to manages the cross-chain conversions between any __ERC20__ tokens on the ethereum blockchain & their pTokenized equivalents on the __EOS__ blockchain. This app implements a simple CLI and a non-HSM-using database in order to serve as an example for how to consume the core library.

&nbsp;

***

&nbsp;

### :earth_africa: App Overview

#### :lock_with_ink_pen: Security:

This demonstration app includes a simple, _unprotected_, __`rocksDB`__ database to serve as an example for how to implement the database interface required by the core library. As such, sans security-features, this app should be used for test-net and similar only.

Main-net implementations will leverage various __HSMs__ in order to provide either an encrypted database interface as is the case with __Strongbox__, or both that and a fully secure runtime environment as is the case with Intel's __SGX__.

&nbsp;

***

&nbsp;

### :point_right: Usage:

```
❍ Provable pERC20-on-EOS Core ❍

    Copyright Provable 2020
    Questions: greg@oraclize.it

❍ Info ❍

This Provable vanilla pERC20-on-EOS app uses the pTokens core in order to manage the cross-chain conversions between
ERC20 ethereum tokens & the pTokenized equivalents on the EOS blockchain.

❍ Usage ❍

Usage:  perc20-on-eos [--help]
        perc20-on-eos [--version]
        perc20-on-eos getEnclaveState
        perc20-on-eos getLatestBlockNumbers
        perc20-on-eos enableEosProtocolFeature <featureHash>
        perc20-on-eos disableEosProtocolFeature <featureHash>
        perc20-on-eos submitEthBlock (<blockJson> | --file=<path>)
        perc20-on-eos submitEosBlock (<blockJson> | --file=<path>)
        perc20-on-eos initializeEos [--chainId=<str>] (<eosJson> | --file=<path>)
        perc20-on-eos initializeEth (<blocksJson> | --file=<path>) (<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        perc20-on-eos debugGetAllDbKeys
        perc20-on-eos debugGetKeyFromDb <key>
        perc20-on-eos debugMigrateContract <ethAddress>
        perc20-on-eos debugAddSupportedToken <ethAddress>
        perc20-on-eos debugSetKeyInDbToValue <key> <value>
        perc20-on-eos debugRemoveSupportedToken <ethAddress>
        perc20-on-eos debugRemoveDictionaryEntry <ethAddress>
        perc20-on-eos debugUpdateIncremerkle (<eosJson> | --file=<path>)
        perc20-on-eos debugReprocessEthBlock (<blockJson> | --file=<path>)
        perc20-on-eos debugReprocessEosBlock (<blockJson> | --file=<path>)
        perc20-on-eos debugAddEosSchedule (<scheduleJson> | --file=<path>)
        perc20-on-eos debugAddDictionaryEntry (<entryJson> | --file=<path>)
        perc20-on-eos signMessageWithEthKey <message>
        perc20-on-eos signHexMsgWithEthKeyWithPrefix <message>
        perc20-on-eos signAsciiMsgWithEthKeyWithNoPrefix <message>

Commands:

    submitEthBlock                      ❍ Submit an ETH block (& its receipts) to the enclave.
                                          ➔ blockJson Format:
                                          {
                                              `block`: The block header itself,
                                              `receipts`: An array containing the block's receipts,
                                              `ref_block_num`: A current EOS reference block number,
                                              `ref_block_prefix`: A current EOS reference block prefix,
                                          }

    submitEosBlock                      ❍ Submit an EOS block (& its receipts) to the enclave.
                                          ➔ blockJson Format:
                                          {
                                              `block_header`: An EOS block header,
                                              `action_proofs`: An array of EOS action proofs,
                                              `interim_block_ids`: An array of EOS block IDs from the core's latest to
                                                                   the block above,
                                          }

    initializeEos                       ❍ Initialize the enclave with the first trusted EOS block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignore by the
                                          enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its EOS
                                          related database from this trusted block, create the EOS private-key and seal
                                          it plus any relevant settings from the `config` into the database. This
                                          command will return a signed transaction to broadcast, which transaction will
                                          deploy the pToken contract to the EOS network.
                                          ➔ blockJson Format:
                                          {
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

    initializeEth                       ❍ Initialize the enclave with the first trusted ETH block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignore by the
                                          enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its ETH
                                          related database from this trusted block, create the ETH private-key and seal
                                          it plus any relevant settings from the `config` into the database. This
                                          command will return a signed transaction to broadcast, which transaction will
                                          deploy the pToken contract to the ETH network.
                                          ➔ blockJson Format: See `submitETHBlock` for format of the JSON.

    enableEosProtocolFeature            ❍ Enable an EOS protocol feature in the core.

    disableEosProtocolFeature           ❍ Disable an EOS protocol feature in the core.

    getEnclaveState                     ❍ Returns the current state of the enclave as pulled from the database.

    debugAddEosSchedule                 ❍ Add an EOS schedule to the database.

    debugGetAllDbKeys                   ❍ Returns JSON formatted report of all the database keys used in the core.


    debugGetKeyFromDb                   ❍ Get a given <key> from the database. This function can only be called if the
                                          `debug` flag is set to true when the tool was built.

    getLatestBlockNumbers               ❍ Returns the current lastest EOS & EOS block numbers seen by the enclave.

    debugMigrateContract                ❍ Create a transaction that will migrate then current balances held by the
                                          `pERC20-on-EOS` smart-contract to the `<ethAddress>` supplied.

    debugSetKeyInDbToValue              ❍ Set a given <key> in the database to a given <value>. This function can only
                                          be called if the `debug` flag is set to true when the core is built. Note that
                                          there are zero checks on what is passed in to the database: Use at own risk!

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

    debugAddSupportedToken              ❍ Returns a signed transaction adding the supplied <ethAddress> as a supported
                                          token to the 'pERC20-on-eos` smart-contract.

    debugRemoveupportedToken            ❍ Returns a signed transaction removing the supplied <ethAddress> as a supported
                                          token from the 'pERC20-on-eos` smart-contract.

    debugReprocessEthBlock              ❍ Reprocess an ETH block by resubmitting it to the core.

    debugReprocessEosBlock              ❍ Reprocess an EOS block by resubmitting it to the core.

    signHexMsgWithEthKeyWithPrefix      ❍ Signs an ASCII message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and NO prefix is
                                          prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signAsciiMsgWithEthKeyWithNoPrefix  ❍ Signs a HEX message with the ETH private key from the encrypted database.
                                          The message is signed via the `secp256k1` signature scheme and the standard
                                          ethereum-specific prefix IS prepended.
                                          Returns: { message: <inputted-message>, signature: <signature> }

    signMessageWithEthKey               ❍ DEPRECATED! This is an alias for `signAsciiMsgWithEthKeyWithNoPrefix`

    <key>                               ❍ A database key in HEX format.

    <value>                             ❍ A database value in HEX format.

    <blockJson>                         ❍ Valid JSON string of EOS or ETH block.

    <message>                           ❍ A message to be signed.

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

    <ethAddress>                        ❍ A valid ethereum address in hex format.

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

    --confs=<uint>                      ❍ The number of confirmations required before signing transactions. This
                                           affects the length of chain the light client maintains in the database.
                                           [default: 0]

    --chainId=<hex|uint>                ❍ Hex string of the EOS chain ID, or integer for ETH chain ID:
                                            1 = Ethereum Main-Net (default)
                                            3 = Ropsten Test-Net
                                            4 = Rinkeby Test-Net
                                            42 = Kovan Test-Net
                                          [default: 1]

    --gasPrice=<uint>                   ❍ The gas price to be used in ETH transactions.
                                          [default: 20000000000]
```

&nbsp;

***

&nbsp;

###  :page_facing_up: Set Up

Enter the __`/scripts`__ directory & you'll find some sample core initialization scripts plus core initialization JSONs. You can use these to initialize the vanilla version of this pToken ERC20-on-EOS bridge thusly:

 - First build the binary with __`cargo b --release`__.

 - Once built, initialize the EOS side of the core by running: __`./init-eos.sh`__

 - Then initialize the ETH side of the core by running: __`./init-eth.sh`__

 - The ETH initialization step will result in an output containing a signed ETH transaction over the supplied smart-contract bytecode, which once broadcast will deploy the __`pERC20-on-eos`__ pToken contract. Before deploying, you'll first need to fund the __`eth-address`__ which you'll also find in the above output. Once that address is funded with some ETH, you may broadcast this transaction.

 - Congratulations, your core is now initialized! You'll know your core is initialized correctly when the command __`<path-to-binary>/perc20_on_eos getEnclaveState`__ returns a JSON containing the core's state.

&nbsp;

***

&nbsp;

### :wrench: Build

You need to ensure you have both __`clang`__ & __`llvm`__ (or later versions) installed on your system. Then enter the __`./app`__ directory and run:

__`❍ cargo build --release`__

To enable __`debug`__ mode in the __`pToken`__ core, do so via the __`Cargo.toml`__ like so:

__`pbtc_core = { path = "<path-to-ptokens-core>", features = ["debug_mode"] }`__

:radioactive: Debug mode __MUST NOT__ be used in production - it bypasses __ALL__ security measures an app may implement, and makes fully accessible the entire database, in plain-text.:radioactive:

#### Versions

 - __`llvm:`__ version 6.0.0 or later.
 - __`clang:`__ version 6.0.0-1ubuntu2 or later.
 - __`rustc & cargo:`__ version 1.42.0-stable or later.

&nbsp;

***

&nbsp;

### :cyclone: Log Rotation

A log for each run of the tool will be written to the __`./logs/`__ directory.

Log rotation occurs when the number of logs reaches the __`MAX_NUM_LOGS`__ threshold. This threshold may be set by the user upon build via an environment variable thusly:

```

MAX_NUM_LOGS=100 cargo b --release

```

__NOTE:__ If no environment variable is provided upon build, the threshold will default to __1000__ logs.

__NOTE:__ The __`MAX_NUM_LOGS`__ also has a lower bound of __20__.

&nbsp;

***

&nbsp;

### :black_nib: Notes

- The maximum __`confs`__ possible during initialization is 255.


&nbsp;

***

&nbsp;

### :guardsman: Tests

To run the tests simply run:

__`❍ cargo test`__

&nbsp;

***

&nbsp;

### :black_nib: To Do:

- [] ETH & EOS debug block reprocessors
