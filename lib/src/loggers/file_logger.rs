use std::{
    env,
    ffi::OsStr,
    fs::{create_dir_all, read_dir, remove_file, File},
    path::{Path, PathBuf},
    time::SystemTime,
};

use log::LevelFilter;
use simplelog::*;

use crate::types::Result;

pub const LOG_DIR_PATH: &str = "logs/";
pub const MINIMUM_MAX_NUM_LOGS: usize = 20;
pub const DEFAULT_MAX_NUM_LOGS: usize = 1000;
pub const LOG_LEVEL_ENV_VAR_KEY: &str = "LOG_LEVEL";
pub const MAX_NUM_LOGS_ENV_VAR_KEY: &str = "MAX_NUM_LOGS";

lazy_static! {
    pub static ref MAX_NUM_LOGS: usize = {
        let default_max_num_logs_string = format!("{}", DEFAULT_MAX_NUM_LOGS);
        let user_selected_max_num_logs = env::var(MAX_NUM_LOGS_ENV_VAR_KEY)
            .unwrap_or(default_max_num_logs_string)
            .parse::<usize>()
            .unwrap_or(DEFAULT_MAX_NUM_LOGS);
        if user_selected_max_num_logs < MINIMUM_MAX_NUM_LOGS {
            info!("✘ User selected < 20 logs, ∴ defaulting to 20!");
            MINIMUM_MAX_NUM_LOGS
        } else {
            user_selected_max_num_logs
        }
    };
}

fn dir_exists(dir: &str) -> bool {
    Path::new(dir).is_dir()
}

fn get_log_file_path(log_dir: &str) -> Result<String> {
    Ok(format!(
        "{}{}.log",
        log_dir,
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis()
    ))
}

fn get_existing_log_file_paths(log_dir: &str) -> Result<Vec<PathBuf>> {
    let mut paths = read_dir(log_dir)?
        .map(|x| -> Result<PathBuf> { Ok(x?.path()) })
        .collect::<Result<Vec<PathBuf>>>()?;
    paths.sort();
    Ok(paths)
}

fn filter_paths_for_log_files(path_bufs: Vec<PathBuf>) -> Vec<PathBuf> {
    path_bufs
        .iter()
        .filter(|path_buf| path_buf.as_path().extension() == Some(OsStr::new("log")))
        .cloned()
        .collect()
}

fn maybe_rotate_logs(logs_dir: &str, max_num_logs: usize) -> Result<()> {
    if !dir_exists(logs_dir) {
        info!("✔ No `{}` dir exists ∴ no logs to rotate!", logs_dir);
        Ok(())
    } else {
        get_existing_log_file_paths(logs_dir)
            .map(filter_paths_for_log_files)
            .and_then(|existing_log_paths| {
                if existing_log_paths.len() > max_num_logs {
                    info!(
                        "✔`{}` directory exists w/ > {} logs ∴ deleting old logs...",
                        logs_dir, max_num_logs
                    );
                    existing_log_paths[..existing_log_paths.len() - max_num_logs]
                        .iter()
                        .try_for_each(|path| {
                            debug!("Removing log @ path: {:?}", path);
                            Ok(remove_file(path)?)
                        })
                } else {
                    info!(
                        "✔`{}` directory exists w/ <= {} logs ∴ not rotating logs!",
                        logs_dir, max_num_logs
                    );
                    Ok(())
                }
            })
    }
}

fn get_log_level_filter_string_from_env_var() -> String {
    env::var(LOG_LEVEL_ENV_VAR_KEY).unwrap_or_else(|_| "Info".to_string())
}

fn get_log_level_filter_from_str(s: &str) -> LevelFilter {
    match s.to_uppercase().as_str() {
        "OFF" => LevelFilter::Off,
        "WARN" => LevelFilter::Warn,
        "DEBUG" => LevelFilter::Debug,
        "ERROR" => LevelFilter::Error,
        "TRACE" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    }
}

fn get_log_level_filter_from_env_var() -> LevelFilter {
    get_log_level_filter_from_str(&get_log_level_filter_string_from_env_var())
}

fn init_file_logger(log_dir: &str) -> Result<()> {
    let log_path = get_log_file_path(log_dir)?;
    if !Path::new(&log_dir).exists() {
        info!("✔ No `{}` directory found, creating...", log_dir);
        create_dir_all(&log_dir)?;
    };
    match WriteLogger::init(
        get_log_level_filter_from_env_var(),
        Config::default(),
        File::create(log_path.clone())?,
    ) {
        Err(e) => Err(e.into()),
        Ok(_) => {
            info!("✔ Logger initialized successfully");
            info!("✔ Writing log to: {}", log_path);
            Ok(())
        },
    }
}

pub fn initialize_file_logger() -> Result<()> {
    init_file_logger(LOG_DIR_PATH).and_then(|_| maybe_rotate_logs(LOG_DIR_PATH, *MAX_NUM_LOGS))
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;

    use super::*;

    #[test]
    fn should_get_log_path() {
        let result = get_log_file_path(LOG_DIR_PATH).unwrap();
        assert!(result.contains("logs/"));
    }

    #[test]
    #[serial]
    fn should_rotate_logs() {
        let test_logs_dir = "test-logs-dir/";
        create_dir_all(&test_logs_dir).unwrap();
        let max_num_logs = 5;
        vec![0; max_num_logs + 1]
            .iter()
            .enumerate()
            .map(|(i, _)| format!("{}file-{}.log", test_logs_dir, i))
            .for_each(|filename| {
                File::create(filename).unwrap();
            });
        let num_log_paths_before = get_existing_log_file_paths(test_logs_dir).unwrap().len();
        assert_eq!(num_log_paths_before, max_num_logs + 1);
        maybe_rotate_logs(test_logs_dir, max_num_logs).unwrap();
        let num_log_paths_after = get_existing_log_file_paths(test_logs_dir).unwrap().len();
        assert_eq!(num_log_paths_after, max_num_logs);
        remove_dir_all(test_logs_dir).unwrap();
    }

    #[test]
    #[serial]
    fn should_rotate_logs_without_removing_non_log_files() {
        let test_logs_dir = "test-logs-dir/";
        create_dir_all(&test_logs_dir).unwrap();
        let max_num_logs = 5;
        vec![0; max_num_logs + 1]
            .iter()
            .enumerate()
            .map(|(i, _)| format!("{}file-{}.log", test_logs_dir, i))
            .for_each(|filename| {
                File::create(filename).unwrap();
            });
        File::create(format!("{}some-file.json", test_logs_dir)).unwrap();
        let num_log_paths_before = get_existing_log_file_paths(test_logs_dir).unwrap().len();
        assert_eq!(num_log_paths_before, max_num_logs + 2);
        maybe_rotate_logs(test_logs_dir, max_num_logs).unwrap();
        let num_log_paths_after = get_existing_log_file_paths(test_logs_dir).unwrap().len();
        assert_eq!(num_log_paths_after, max_num_logs + 1);
        remove_dir_all(test_logs_dir).unwrap();
    }

    #[test]
    fn max_num_logs_should_default_to_one_thousand() {
        assert!(env::var(MAX_NUM_LOGS_ENV_VAR_KEY).is_err());
        assert_eq!(*MAX_NUM_LOGS, DEFAULT_MAX_NUM_LOGS);
    }

    #[test]
    fn should_get_log_level_filter_from_str() {
        let expected_result = LevelFilter::Trace;
        let result = get_log_level_filter_from_str("TRACE");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_log_level_filter_string_from_env_var() {
        let env_var_value = "an env var value".to_string();
        env::set_var(LOG_LEVEL_ENV_VAR_KEY, env_var_value.clone());
        let result = get_log_level_filter_string_from_env_var();
        assert_eq!(result, env_var_value);
    }

    #[test]
    fn should_get_log_level_filter_from_env_var() {
        let env_var_value = "trace".to_string();
        env::set_var(LOG_LEVEL_ENV_VAR_KEY, env_var_value.clone());
        let expected_result = LevelFilter::Trace;
        let result = get_log_level_filter_from_env_var();
        assert_eq!(result, expected_result);
    }
}
