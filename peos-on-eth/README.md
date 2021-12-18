# :closed_lock_with_key: pEOS-on-ETH App

This Provable __pEOS-on-ETH__ app leverages the __pToken__ core in order to manages the cross-chain conversions between native EOS tokens and the __pTokenied__ version on the ethereum block chain.

&nbsp;

***

&nbsp;

### :earth_africa: App Overview

#### :lock_with_ink_pen: Security:

This demonstration app includes a simple, _unprotected_, __`rocksDB`__ database to serve as an example for how to implement the database interface required by the core library. As such this app should be used for test-net and similar only.

Main-net implementations will leverage various __HSMs__ in order to provide either an encrypted database interface as is the case with __Strongbox__, or both that and a fully secure runtime environment as is the case with Intel's __SGX__.

&nbsp;

***

&nbsp;

### :point_right: Usage:

```


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
 - __`rustc & cargo:`__ version 1.42.0 or later.

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
