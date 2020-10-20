use sha3::{Digest, Sha3_256};
use md5::Md5;
use blake2::Blake2b;
use bytes::Bytes;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // we take command line arguments and define behavior
    // based on it
    
    // method of encryption comes first

    let method = &args[1];

    // then filename/path
    
    let filepath = &args[2];

    let input = fs::read_to_string(filepath)
        .expect("File read failed.");

    //convert plaintext to bytes for digest

    match method.as_str() {
        "MD5"    => hash_md5(input),
        "SHA3"   => hash_sha3(input),
        "Blake2" => hash_blake(input),
        _        => print_error_message(),
    };
}

fn print_error_message() {
    println!("Hashing method unrecognized. Options:");
    println!("  MD5     SHA3    Blake2");
}

fn hash_md5(plaintext: String) {
    let mut hasher = Md5::new();
    let plaintext = Bytes::from(plaintext);

    hasher.update(plaintext);

    println!("MD5: {:x}", hasher.finalize());
}

fn hash_sha3(plaintext: String) {
    let mut hasher = Sha3_256::new();
    let plaintext = Bytes::from(plaintext);

    hasher.update(plaintext);

    println!("SHA3_256: {:x}", hasher.finalize());
}

fn hash_blake(plaintext: String) {
    let mut hasher = Blake2b::new();
    let plaintext = Bytes::from(plaintext);

    hasher.update(plaintext);

    println!("Blake2: {:x}", hasher.finalize());
}
