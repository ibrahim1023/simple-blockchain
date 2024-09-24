use std::sync::Arc;

use tokio::{ task, join };
use tokio::sync::Mutex;

mod blockchain;

use blockchain::{ generate_random_transaction, Blockchain };

#[tokio::main]
async fn main() {
    run_blockchain().await;
}

pub async fn run_blockchain() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    let transactions1 = generate_random_transaction();
    let transactions2 = generate_random_transaction();
    let transactions3 = generate_random_transaction();

    let blockchain1 = Arc::clone(&blockchain);
    let blockchain2 = Arc::clone(&blockchain);
    let blockchain3 = Arc::clone(&blockchain);

    let task1 = task::spawn(async move {
        let mut blockchain = blockchain1.lock().await;
        blockchain.add_block(transactions1).await;
    });

    let task2 = task::spawn(async move {
        let mut blockchain = blockchain2.lock().await;
        blockchain.add_block(transactions2).await;
    });

    let task3 = task::spawn(async move {
        let mut blockchain = blockchain3.lock().await;
        blockchain.add_block(transactions3).await;
    });

    let _ = join!(task1, task2, task3);

    let blockchain = blockchain.lock().await;
    for block in &blockchain.chain {
        println!("{:?}", block);
    }
}
