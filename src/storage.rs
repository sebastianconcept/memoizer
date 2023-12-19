#![deny(unsafe_code)]
/* No `unsafe` needed! */

use mut_static::MutStatic;
use rand::seq::SliceRandom;
use std::fs::OpenOptions;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

use twox_hash::xxh3::RandomHashBuilder64;

#[derive(Clone)]
pub struct Storage {
    pub store: HashMap<String, String, RandomHashBuilder64>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            store: Default::default(),
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

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    pub static ref STORAGE: MutStatic<Storage> = MutStatic::from(Storage::new());
    pub static ref KEYS: MutStatic<Vec<String>> = MutStatic::from(vec!());
    pub static ref SAMPLE_VALUE: String = "{\"hlrSgsnNumber\":null,\"sponsoredImsi\":\"525053099536133\",\"vlrMscNumber\":\"792411112905\",\"mnc\":\"02\",\"vlrVlrNumber\":\"792411112905\",\"_id\":\"28981640290848413548099571056\",\"hlrMscNumber\":\"804107924111122\",\"#version\":-928585930571132360,\"hlrScfAddress\":\"14174000087\",\"customerImsi\":\"312300000591679\",\"sponsorName\":\"IMSI10\",\"sponsoredId\":\"10\",\"updatedTime\":\"2019-10-15T00:04:28.483+00:00\",\"hlrVlrNumber\":\"804107924111121\",\"maxGTLength\":15,\"rhToVLRGT\":\"6598541000\",\"vlrCalledTranslationType\":0,\"mme\":null,\"customerMsisdn\":\"879000000591679\",\"mcc\":\"250\",\"pilotMode\":0,\"skipCancelLocation\":null,\"packetSwitched\":false,\"sponsoredMsisdn\":\"65985001136133\",\"vlrSgsnNumber\":null,\"hlrHlrNumber\":\"14174000019\",\"mtSmsRewriteV1\":null,\"creationTime\":\"2019-10-15T00:04:28.483+00:00\",\"#instanceOf\":\"RHVlrImsiMapping\"}".to_string();
}

pub static OUTPUT_FILE_NAME: &str = "output.txt";

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
    writeln!(file, "{}", contents).unwrap();
    println!("{}", contents);
    file.flush().unwrap();
}

fn bench_inserts(quantity: usize, sample_payload: String) {
    let bench_result = benchmarking::measure_function(move |measurer| {
        measurer.measure(|| {
            output(format!("Measuring Rust HashMap {} inserts...", quantity));
            for _i in 0..quantity {
                let id = Uuid::new_v4().to_string();
                let value = format!("{}-{}", id, sample_payload);
                KEYS.write()
                    .expect("Failed to grab a lock to mutate the KEYS object")
                    .push(id.clone());
                basic_set(id.to_owned(), value);
            }
        });
    })
    .unwrap();
    let contents = format!(
        "It took {:?} to perform {} insertions",
        bench_result.elapsed(),
        quantity
    );
    output(contents);
}

fn bench_reads(quantity: usize) {
    let bench_result = benchmarking::measure_function(move |measurer| {
        measurer.measure(|| {
            output(format!("Measuring Rust HashMap {} reads...", quantity));
            for _i in 0..quantity {
                let key = basic_keys_any();
                let _value = basic_get(key.clone()).unwrap();
            }
        });
    })
    .unwrap();
    let contents = format!(
        "It took {:?} to perform {} reads",
        bench_result.elapsed(),
        quantity
    );
    output(contents);
}

/// Run a basic benchmaking on writes and reads on the storage using an optional payload.
pub fn benchmark(quantity: usize, sample_payload: Option<String>) {
    reset_output();
    output("Starting the benchmarking...".to_string());
    benchmarking::warm_up();
    output("Benchmarking warmed up and ready to go.".to_string());
    match sample_payload {
        Some(s) => bench_inserts(quantity, s),
        None => bench_inserts(quantity, SAMPLE_VALUE.clone()),
    }
    bench_reads(quantity);
}

pub fn keys_size() -> u32 {
    KEYS.read()
        .expect("Failed to grab a lock to read the KEYS object")
        .len()
        .try_into()
        .unwrap()
}

fn basic_keys_any() -> String {
    KEYS.read()
        .expect("Failed to grab a lock to read the KEYS object")
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
}

pub fn keys_any() -> String {
    basic_keys_any()
}

pub fn size() -> i32 {
    STORAGE
        .read()
        .expect("Failed to grab a lock to access the Storage object")
        .size()
}

pub fn reset() {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .reset();
}

pub fn includes(key: String) -> bool {
    STORAGE
        .read()
        .expect("Failed to grab a lock to read in the Storage object")
        .includes(key.to_string().clone())
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
