#[derive(Debug, Clone)]
pub struct Block {
  pub timestamp: i64,
  pub message: String,
  pub nonce: i128,
  pub previous_hash: String
}
