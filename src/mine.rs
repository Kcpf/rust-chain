use chrono::Utc;
use sha2::{Sha256, Digest};

use crate::classes::block::Block;
use crate::classes::blockchain::Blockchain;


pub fn mine(mut chain: Blockchain, message: String) -> Blockchain {
  let timestamp = Utc::now().timestamp();
  let previous_hash = chain.get_last_block_hash().clone();
  
  let number_of_zeros = "0".repeat(chain.difficulty.try_into().unwrap());
  let mut hasher: Sha256;
  let mut nonce: i128 = 0;
  let mut string_to_hash: String;
  
  loop {
    string_to_hash = format!("{}|{}|{}|{}", timestamp, message, nonce, previous_hash);
  
    hasher = Sha256::new();
    hasher.update(string_to_hash.as_bytes());
    
    let block_hash = format!("{:x}", hasher.finalize());

    if block_hash.starts_with(&number_of_zeros) { break }
    
    nonce += 1 
  }

  let new_block = Block {
    timestamp: timestamp, 
    message: message,
    nonce: nonce,
    previous_hash: previous_hash
  };

  chain.proof_of_work(new_block);

  return chain;
}
