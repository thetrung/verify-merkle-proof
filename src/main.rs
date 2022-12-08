use sha2::{Digest, Sha256};

const ACCOUNT_CODE: &str = "8dc20f34da8cea8dd0f46b001694f5123ecd30d786c5eb92ad1a013703a4f8d1";
const SALT: &str = "AB12C34DEFG5KSQI";
const AUDIT_ID: &str = "PR30JUN22";
const BALANCES: &str = "ADA:15129.4,ADA.S:0.0,BTC:0.2600852178,BTC.M:1.25,DOT:50.0,DOT.S:20.5,DOT.P:0.0,ETH:5.27518778,ETH2.S:10.123,USDC:50000.0,USDT:0.0,XRP:0.000002";

fn main() {

    // Record ID
    let mut record_hasher: Sha256 = Default::default();
    record_hasher.update(ACCOUNT_CODE);
    record_hasher.update(SALT);
    record_hasher.update(AUDIT_ID);
    let record_id: String = format!("{:x}", record_hasher.finalize());
    println!("Record ID: {}", record_id);

    // Merkle Hash
    let merkle_hash: String = format!("{},{}", record_id, BALANCES);
    println!("Merkle Hash: {}", merkle_hash);

    // SHA Result
    let mut merkle_hasher: Sha256 = Default::default();
    merkle_hasher.update(&merkle_hash);
    let merkle_result: String = format!("{:x}", merkle_hasher.finalize());
    println!("SHA Result: {}", merkle_result);

    // Merkle Leaf
    let merkle_leaf: &str = &merkle_result[..16];
    println!("Merkle Leaf: {}", &merkle_leaf);
}