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

    if args.len() < 2 {
        panic!("Not enough arguments.");
    }

    let method = &args[1];

    match method.as_str() {
        "--help"    => print_help_message(),
        "-h"        => print_help_message(),
        _           => (),                      //no behavior if not help option
    };

    if args.len() < 3 {
        panic!("No filepath specified.");
    }

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

fn print_help_message() {
    println!("hash-gen usage:");
    println!("./hash-gen [OPTIONS] [FILEPATH]");
    println!("");
    println!("OPTIONS:");
    println!("  MD5");
    println!("  SHA3");
    println!("  Blake2");
    println!("");
    println!("to bring up this message, use --help | -h ");
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
