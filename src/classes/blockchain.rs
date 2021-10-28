use chrono::Utc;
use sha2::{Sha256, Digest};

use crate::classes::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
  pub chain: Vec<Block>,
  pub difficulty: i8,
}

impl Blockchain {

  pub fn new() -> Blockchain {
    let genesis_block = Block {
      timestamp: Utc::now().timestamp(),
      message: "Genesis Block".to_string(),
      nonce: 0,
      previous_hash: "".to_string()
    };

    return Blockchain {
      chain: vec![genesis_block],
      difficulty: 2
    };
  }

  fn create_new_block(&mut self, block: Block) {
    self.chain.push(block);
    self.change_difficulty();
  }

  fn change_difficulty(&mut self) {
    self.difficulty += 1;
  }

  fn get_hash_from_block(&self, block: &Block) -> String {
    let timestamp = block.timestamp;
    let message = block.message.clone();
    let nonce = block.nonce;
    let previous_hash = block.previous_hash.clone();

    let string_to_hash = format!("{}|{}|{}|{}", timestamp, message, nonce, previous_hash);
    
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(string_to_hash.as_bytes());
    let result = hasher.finalize();

    return format!("{:x}", result);
  }

  fn get_last_block(&self) -> &Block {
    return self.chain.last().unwrap();
  }

  pub fn get_last_block_hash(&self) -> String {
    let last_block = self.get_last_block();

    return self.get_hash_from_block(last_block);
  }

  pub fn proof_of_work(&mut self, block: Block) -> bool {
    let block_hash = self.get_hash_from_block(&block);

    let number_of_zeros = "0".repeat(self.difficulty.try_into().unwrap());

    if block_hash.starts_with(&number_of_zeros) && block.previous_hash == self.get_last_block_hash() {
      self.create_new_block(block);
      return true;
    }

    return false;
  }

  pub fn chain_valid(&self) -> bool {
    for block_index in 1..self.chain.len() {
      let last_block = self.chain[block_index - 1].clone();

      if self.chain[block_index].previous_hash != self.get_hash_from_block(&last_block) {
        return false;
      }
    }

    return true;
  }
}
