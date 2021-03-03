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
        SerdeJsonError(err: serde_json::Error) {
            from()
            display("✘ Serde-Json Error!\n✘ {}", err)
        }
    }
}

#[cfg(not(feature = "json-rpc-db"))]
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
        RocksDbError(err: rocksdb::Error) {
            from()
            display("✘ Rocks DB error: {}", err)
        }
    }
}
