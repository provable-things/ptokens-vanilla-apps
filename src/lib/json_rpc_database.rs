use crate::lib::{
    types::{DataSensitivity, Result},
    utils::maybe_strip_hex_prefix,
};
use ptokens_core::{AppError as PTokensCoreError, Bytes, DatabaseInterface, Result as PTokensResult};
use reqwest::blocking::Client;
use serde_json::{json, Value as JsonValue};
use std::{cell::RefCell, collections::HashMap, env};

pub struct Database {
    pub url: String,
    pub batch_db_ops: RefCell<Vec<DbOp>>,
    pub keys_to_delete: RefCell<Vec<Bytes>>,
    pub hashmap: RefCell<HashMap<Bytes, Bytes>>,
}

pub enum DbOp {
    Delete(Bytes),
    Put(Bytes, Bytes, u8),
}

impl DbOp {
    pub fn to_json(&self) -> JsonValue {
        match self {
            DbOp::Delete(key) => json!({"method": "delete", "key": hex::encode(&key) }),
            DbOp::Put(key, value, sensitivity) => json!({
                "method": "put",
                "key": hex::encode(&key),
                "sensitivity": sensitivity,
                "value": hex::encode(&value)
            }),
        }
    }
}

impl Database {
    pub fn open() -> Result<Self> {
        Ok(Self {
            url: env::var("JSON_RPC_HOST")?,
            hashmap: RefCell::new(HashMap::new()),
            batch_db_ops: RefCell::new(Vec::new()),
            keys_to_delete: RefCell::new(Vec::new()),
        })
    }
}

fn make_simple_json_rpc_call(url: &str, json: JsonValue, err_msg: &str) -> PTokensResult<()> {
    match Client::new().post(url).json(&json).send() {
        Err(e) => Err(PTokensCoreError::Custom(e.to_string())),
        Ok(body) => match body.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => Err(PTokensCoreError::Custom(format!(
                "{} Json RPC status code: {}",
                err_msg,
                body.status().to_string()
            ))),
        },
    }
}

fn extract_bytes_from_json_result(json: JsonValue) -> PTokensResult<Bytes> {
    match json.get("result") {
        None => Err(PTokensCoreError::Custom("No `result` key in json!!".into())),
        Some(json_value) => match json_value {
            JsonValue::Null => Err(PTokensCoreError::Custom("`Null` result found in db!".into())),
            JsonValue::String(value) => match hex::decode(maybe_strip_hex_prefix(&value)) {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(PTokensCoreError::Custom(format!(
                    "Error decoding hex from `get` operation: {}",
                    e
                ))),
            },
            _ => Err(PTokensCoreError::Custom("Json value is wrong type!".into())),
        },
    }
}

fn extract_sensitivity_value(sensitivity: Option<u8>) -> u8 {
    match sensitivity {
        None => 0,
        Some(value) => value,
    }
}

impl DatabaseInterface for Database {
    fn end_transaction(&self) -> PTokensResult<()> {
        info!("✔ Ending DB transaction in app...");
        let json = json!({
            "jsonrpc": "2.0",
            "method": "end_transaction",
            "params": self.batch_db_ops.borrow().iter().map(|db_op| db_op.to_json()).collect::<Vec<JsonValue>>(),
        });
        info!("✔ Batch writing to DB...");
        make_simple_json_rpc_call(&self.url, json, "Error ending DB transaction!")
    }

    fn start_transaction(&self) -> PTokensResult<()> {
        info!("✔ Starting DB transaction in app...");
        let json = json!({
            "jsonrpc": "2.0",
            "method": "start_transaction",
            "params": [],
        });
        make_simple_json_rpc_call(&self.url, json, "Error starting DB transaction!")
    }

    fn put(&self, key: Bytes, value: Bytes, sensitivity: DataSensitivity) -> PTokensResult<()> {
        trace!("✔ Putting key in hashmap...");
        self.hashmap.borrow_mut().insert(key.clone(), value.clone());
        trace!("✔ Checking if key is in delete list... ");
        match self.keys_to_delete.borrow().contains(&key) {
            false => trace!("✔ Key not in delete list, nothing to remove."),
            true => {
                trace!("✔ Removing key from delete list... ");
                self.hashmap.borrow_mut().remove(&key);
            },
        };
        self.batch_db_ops
            .borrow_mut()
            .push(DbOp::Put(key, value, extract_sensitivity_value(sensitivity)));
        Ok(())
    }

    fn delete(&self, key: Bytes) -> PTokensResult<()> {
        trace!("✔ Removing key from hashmap...");
        self.hashmap.borrow_mut().remove(&key);
        trace!("✔ Adding key to `to_delete` list...");
        self.keys_to_delete.borrow_mut().push(key.clone());
        self.batch_db_ops.borrow_mut().push(DbOp::Delete(key));
        Ok(())
    }

    fn get(&self, key: Bytes, sensitivity: DataSensitivity) -> PTokensResult<Bytes> {
        let not_in_db_error = "✘ Cannot find item in database!".to_string();
        match self.keys_to_delete.borrow().contains(&key) {
            true => {
                info!("✔ Key already in delete list ∴ 'not found'!");
                Err(PTokensCoreError::Custom(not_in_db_error))
            },
            false => {
                trace!("✔ Checking hashmap for key...");
                match self.hashmap.borrow().get(&key) {
                    Some(value) => Ok(value.to_vec()),
                    None => {
                        let json = json!({
                            "jsonrpc": "2.0",
                            "method": "get",
                            "params": [ json!({
                                "key": hex::encode(&key),
                                "sensitivity": extract_sensitivity_value(sensitivity)
                            })],
                        });
                        info!("✘ Key NOT in hashmap! Looking in underlying DB...");
                        match Client::new().post(&self.url).json(&json).send() {
                            Err(e) => Err(PTokensCoreError::Custom(e.to_string())),
                            Ok(body) => match body.status() {
                                reqwest::StatusCode::OK => match body.json::<JsonValue>() {
                                    Ok(json) => extract_bytes_from_json_result(json),
                                    Err(e) => Err(PTokensCoreError::Custom(e.to_string())),
                                },
                                _ => Err(PTokensCoreError::Custom(format!(
                                    "Error getting bytes from db! Json RPC status code: {}",
                                    body.status().to_string()
                                ))),
                            },
                        }
                    },
                }
            },
        }
    }
}
