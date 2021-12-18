// NOTE: The duplication here is to avoid compiling RocksDB when it unneeded ∵ it takes so long to build
#[cfg(feature = "json-rpc-db")]
quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ Program Error: {}", err)
        }
        IOError(err: std::io::Error) {
            from()
            display("✘ I/O Error: {}", err)
        }
        SystemTimeErorr(err: std::time::SystemTimeError) {
            from()
            display("✘ System time error: {}", err)
        }
        PTokenAppError(err: ptokens_core::errors::AppError) {
            from()
            display("✘ PToken core error: {}", err)
        }
        EnvError(err: std::env::VarError) {
            from()
            display("✘ Env var error: {}", err)
        }
        SetLoggerError(err: log::SetLoggerError) {
            from()
            display("✘ SetLogger error: {}", err)
        }
        ParseIntError(err: std::num::ParseIntError) {
            from()
            display("✘ Parse Int error: {}", err)
        }
        DocoptError(err: docopt::Error) {
            from()
            display("✘ Docopt error: {}", err)
        }
        SerdeJsonError(err: serde_json::Error) {
            from()
            display("✘ Serde-Json Error: {}", err)
        }
    }
}

#[cfg(feature = "rocks-db")]
quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ Program Error: {}", err)
        }
        IOError(err: std::io::Error) {
            from()
            display("✘ I/O Error: {}", err)
        }
        SystemTimeErorr(err: std::time::SystemTimeError) {
            from()
            display("✘ System time error: {}", err)
        }
        PTokenAppError(err: ptokens_core::errors::AppError) {
            from()
            display("✘ PToken core error: {}", err)
        }
        EnvError(err: std::env::VarError) {
            from()
            display("✘ Env var error: {}", err)
        }
        SetLoggerError(err: log::SetLoggerError) {
            from()
            display("✘ SetLogger error: {}", err)
        }
        DocoptError(err: docopt::Error) {
            from()
            display("✘ Docopt error: {}", err)
        }
        ParseIntError(err: std::num::ParseIntError) {
            from()
            display("✘ Parse Int error: {}", err)
        }
        RocksDbError(err: rocksdb::Error) {
            from()
            display("✘ Rocks DB error: {}", err)
        }
    }
}
