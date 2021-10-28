mod classes;
mod mine;

use crate::classes::blockchain::Blockchain;

fn main() {
  let mut chain = Blockchain::new();

  chain = mine::mine(chain, "Bloco 1".to_string());
  chain = mine::mine(chain, "Bloco 2".to_string());
  
  println!("{:?}", chain);
  println!("{}", chain.chain_valid());
}
