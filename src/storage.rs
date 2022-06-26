#![deny(unsafe_code)]
/* No `unsafe` needed! */


use mut_static::MutStatic;
use rand::seq::SliceRandom;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::{Error, Write};
use std::os::raw::c_uint;
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::{
    collections::HashMap,
    ffi::CString,
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Storage {
    pub store: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
    pub fn includes(&self, key: String) -> bool {
        self.store.contains_key(&key)
    }
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
    pub fn reset(&mut self) {
        self.store.clear()
    }
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    pub fn size(&self) -> i32 {
        self.store.len().try_into().unwrap()
    }
}

lazy_static! {
    pub static ref STORAGE: MutStatic<Storage> = MutStatic::from(Storage::new());
    pub static ref KEYS: MutStatic<Vec<String>> = MutStatic::from(vec!());
}

pub static OUTPUT_FILE_NAME: &str = "output.txt";

pub static SAMPLE_VALUE: &str = "{\"hlrSgsnNumber\":null,\"sponsoredImsi\":\"525053099536133\",\"vlrMscNumber\":\"792411112905\",\"mnc\":\"02\",\"vlrVlrNumber\":\"792411112905\",\"_id\":\"28981640290848413548099571056\",\"hlrMscNumber\":\"804107924111122\",\"#version\":-928585930571132360,\"hlrScfAddress\":\"14174000087\",\"customerImsi\":\"312300000591679\",\"sponsorName\":\"IMSI10\",\"sponsoredId\":\"10\",\"updatedTime\":\"2019-10-15T00:04:28.483+00:00\",\"hlrVlrNumber\":\"804107924111121\",\"maxGTLength\":15,\"rhToVLRGT\":\"6598541000\",\"vlrCalledTranslationType\":0,\"mme\":null,\"customerMsisdn\":\"879000000591679\",\"mcc\":\"250\",\"pilotMode\":0,\"skipCancelLocation\":null,\"packetSwitched\":false,\"sponsoredMsisdn\":\"65985001136133\",\"vlrSgsnNumber\":null,\"hlrHlrNumber\":\"14174000019\",\"mtSmsRewriteV1\":null,\"creationTime\":\"2019-10-15T00:04:28.483+00:00\",\"#instanceOf\":\"RHVlrImsiMapping\"}";

/*
   Storage and its API
*/
fn reset_output() {
    if Path::new(OUTPUT_FILE_NAME).exists() {
        fs::remove_file(OUTPUT_FILE_NAME).unwrap();
    }
    File::create(OUTPUT_FILE_NAME).unwrap();
}

fn output(contents: String) {
    let is_new = Path::new(OUTPUT_FILE_NAME).exists();
    let mut file = OpenOptions::new()
        .create(!is_new)
        .write(true)
        .append(true)
        .open(OUTPUT_FILE_NAME)
        .unwrap();
    write!(file, "{}\n", contents).unwrap();
    println!("{}", contents);
    file.flush().unwrap();
}


pub fn keys_size() -> u32 {
    KEYS.read()
        .expect("Failed to grab a lock to read the KEYS object")
        .len()
        .try_into()
        .unwrap()
}

fn basic_keys_any() -> String {
    let keys = KEYS
        .read()
        .expect("Failed to grab a lock to read the KEYS object");
    keys.choose(&mut rand::thread_rng()).unwrap().to_owned()
}

pub fn keys_any() -> String {
    basic_keys_any()
}

pub fn size() -> i32 {
    let storage = STORAGE
        .read()
        .expect("Failed to grab a lock to access the Storage object");
    storage.size()
}

pub fn reset() {
    let mut storage = STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object");
    storage.reset();
}

pub fn includes(key: String) -> bool {
    let storage = STORAGE
        .read()
        .expect("Failed to grab a lock to read in the Storage object");
    storage.includes(key.to_string().clone())
}

pub fn remove_key(key: String) {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .remove(key.to_string());
}

fn basic_set(key: String, value: String) {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .set(key, value);
}

pub fn set(key: String, value: String) {
    basic_set(key, value);
}

fn basic_get(key: String) -> Option<String> {
    STORAGE
        .read()
        .expect("Failed to grab a lock to read in the Storage object")
        .get(key)
}

pub fn get(key: String) -> Option<String> {
  basic_get(key)
}

// pub fn echo(key: String) -> String {
//     let answer = String::from(key.to_str());
//     answer.try_into().unwrap()
// }
