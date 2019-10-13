extern crate tempdir;
extern crate leveldb;

use tempdir::TempDir;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options,WriteOptions,ReadOptions};

mod merkle_mountain_range;

fn main() {
    // let digest = hashing_function;
    // let db = database;
    merkle_mountain_range::serialize();
}
