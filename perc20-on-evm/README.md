# :closed_lock_with_key: pERC20-on-EVM App

This Provable __pERC20-on-EVM__ app leverages the __pToken__ core in order to manage the cross-chain conversions between __ERC20__ tokens & their __pTokenized__ equivalents on another EVM-compatible blockchain. This app implements a simple CLI and a non-HSM-using database in order to serve as an example for how to consume the core library.

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
        perc20-on-evm initializeEvm (<blocksJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        perc20-on-evm initializeEth (<blocksJson> | --file=<path>) [--chainId=<uint>] [--gasPrice=<uint>] [--confs=<uint>]
        perc20-on-evm debugGetAllDbKeys
        perc20-on-evm debugGetKeyFromDb <key>
        perc20-on-evm debugMigrateContract <ethAddress>
        perc20-on-evm debugAddSupportedToken <ethAddress>
        perc20-on-evm debugSetKeyInDbToValue <key> <value>
        perc20-on-evm debugRemoveSupportedToken <ethAddress>
        perc20-on-evm debugRemoveDictionaryEntry <ethAddress>
        perc20-on-evm debugReprocessEthBlock (<blockJson> | --file=<path>)
        perc20-on-evm debugReprocessEvmBlock (<blockJson> | --file=<path>)
        perc20-on-evm debugAddDictionaryEntry (<entryJson> | --file=<path>)
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
                                          ➔ blocksJson Format: See `submitEthBlock` for breakdown of JSON.

    initializeEvm                       ❍ Initialize the enclave with the first trusted EVM block. Ensure the block has
                                          NO transactions relevant to the pToken in it, because they'll be ignored by
                                          the enclave. Transactions are not verified so you may omit them and include an
                                          empty array in their place if needs be. The enclave will initialize its ETH
                                          related database from this trusted block, create the ETH private-key and seal
                                          it plus any relevant settings from the `config` into the database.
                                          This command will return a signed transaction to broadcast, which transaction
                                          will deploy the pToken contract to the ETH network.
                                          ➔ blocksJson Format: See `submitEvmBlock` for breakdown of JSON.

    getEnclaveState                     ❍ Returns the current state of the enclave as pulled from the database.

    addVaultContractAddress             ❍ Adds the ERC20 vault contract address to the encrypted database.

    debugGetAllDbKeys                   ❍ Returns JSON formatted report of all the database keys used in the core.

    debugMigrateContract                ❍ Create a transaction that will migrate then current balances held by the
                                          `ERC20-vault` smart-contract to the `<ethAddress>` supplied.

    debugGetKeyFromDb                   ❍ Get a given <key> from the database. This function can only be called if the
                                          `debug` flag is set to true when the tool was built.


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

    debugSetKeyInDbToValue              ❍ Set a given <key> in the database to a given <value>. This function can only
                                          be called if the `debug` flag is set to true when the core is built. Note that
                                          there are zero checks on what is passed in to the database: Use at own risk!

    debugAddDictionaryEntry             ❍ Add a dictionary entry.

    debugRemoveDictionaryEntry          ❍ Remove a dictionary entry via its ETH address.


    <key>                               ❍ A database key in HEX format.

    <value>                             ❍ A database value in HEX format.

    <blockJson>                         ❍ Valid JSON string of ETH or BTC block.

    <path>                              ❍ Path to the file being submitted to the app.

    <message>                           ❍ A message to be signed.

    <ethAddress>                        ❍ A valid ethereum address in hex format.

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

    --nonce=<uint>                      ❍ Transaction nonce

    --ethNetwork=<str>                  ❍ Transaction network name
                                            - mainnet
                                            - ropsten
                                            - rinkeby
                                            - kovan

    --gasPrice=<uint>                   ❍ Transaction gas price
```

&nbsp;

***

&nbsp;

### :wrench: Build

You need to ensure you have both __`clang`__ & __`llvm`__ (or later versions) installed on your system. Then enter the __`./app`__ directory and run:

__`❍ cargo build --release`__

To enable __`debug`__ mode in the __`pToken`__ core, do so via the __`Cargo.toml`__ like so:

__`pbtc_core = { path = "../core", features = ["debug_mode"] }`__

:radioactive: Debug mode __MUST NOT__ be used in production - it bypasses __ALL__ security measures an app may implement, and makes fully accessible the entire database, in plain-text.:radioactive:

#### Versions

 - __`llvm:`__ version 6.0.0 or later.
 - __`clang:`__ version 6.0.0-1ubuntu2 or later.
 - __`rustc & cargo:`__ version 1.42.0-nightly or later.

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

- When initializing the enclave, the merkle-roots inside the __ETH__ & __EVM__ blocks are __NOT__ verified - only the block headers are checked. For smaller initialiazation material, feel free to provide empty arrays for the transactions. Ensure no relevant transactions took place in the blocks used to initialize the enclave.

&nbsp;

***

&nbsp;

### :guardsman: Tests

To run the tests simply run:

__`❍ cargo +nightly test`__

&nbsp;

***

&nbsp;

### :black_nib: To Do:

