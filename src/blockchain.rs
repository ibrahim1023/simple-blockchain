use rand::Rng;
use serde::{ Serialize, Deserialize };
use tokio::time::{ sleep, Duration };
use blake3::hash;
use std::time::{ SystemTime, UNIX_EPOCH };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub transactions: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub async fn new(index: u32, transactions: String, previous_hash: String) -> Self {
        let timestamp = current_time();

        sleep(Duration::from_millis(500)).await;

        let block_data = format!("{}{}{}{}", index, timestamp, transactions, previous_hash);
        let hash = hash_block(&block_data);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
        }
    }

    fn create_genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: current_time(),
            transactions: "Genesis Block".to_string(),
            previous_hash: String::from("0"),
            hash: hash_block("Genesis Block"),
        }
    }

    pub async fn add_block(&mut self, transactions: String) {
        let latest_block = self.chain.last().unwrap();

        let new_block = Block::new(
            latest_block.index + 1,
            transactions,
            latest_block.hash.clone()
        ).await;

        self.chain.push(new_block);
    }
}

fn current_time() -> u128 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
}

fn hash_block(data: &str) -> String {
    hash(data.as_bytes()).to_hex().to_string()
}

pub fn generate_random_transaction() -> String {
    let mut rng = rand::thread_rng();
    let amount: u32 = rng.gen_range(1..100);
    let from: u32 = rng.gen_range(1000..9999);
    let to: u32 = rng.gen_range(1000..9999);

    format!("{{ from: {}, to: {}, amount: {} }}", from, to, amount)
}
