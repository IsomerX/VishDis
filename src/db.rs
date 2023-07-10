use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Db {
    entries: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];
        let val = value.clone();
        let res = self.entries.lock().unwrap().insert(key.clone(), val.into());

        match res {
            Some(_res) => Ok("r ok"),
            None => Ok("ok"),
        }
    }

    pub fn read(&self, arr: &[String]) -> Result<Bytes, &'static str> {
        let key = &arr[1];
        let binding = self.entries.lock().unwrap();
        let query_result = binding.get(key);

        match query_result {
            Some(value) => Ok(value.clone()),
            None => Err("Key not found"),
        }
    }

    pub fn clone(&self) -> Self {
        Db {
            entries: self.entries.clone(),
        }
    }
}
