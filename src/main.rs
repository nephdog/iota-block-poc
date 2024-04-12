//! This examples shows how to connect to a client node, add a tagged data block to the tangle, and read from that block.
//!
//! ```sh
//! cargo run --release --example client_getting_started
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::{Block, BlockId},
    types::block::payload::Payload,
};

async fn connect_to_client_node(node_url: &String) -> Result<Client> {
    println!("Connecting to client node");

    let client = Client::builder()
    .with_node(&node_url)? // Insert your node URL here
    .finish()
    .await?;

    let info = client.get_info().await?;
    println!("Node Info: {info:?}");

    Ok(client)
}

async fn add_tagged_data_block(client: &Client, tag: &String, data: &String, explorer_url: &String) -> Result<Block> {
    println!("Adding tagged data block");

    let block = client
        .build_block()
        .with_tag(tag.as_bytes().to_vec())
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await?;

    println!("{block:#?}\n");

    if let Some(Payload::TaggedData(payload)) = block.payload() {
        println!(
            "Tag: {}",
            String::from_utf8(payload.tag().to_vec()).expect("found invalid UTF-8")
        );
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("found invalid UTF-8")
        );
    }

    println!(
        "Block with tag and data sent: {}/block/{}",
        explorer_url,
        block.id()
    );

    Ok(block)
}

async fn retrieve_block_by_id(client: &Client, block_id: &BlockId) -> Result<Block> {
    println!("Retrieving tagged block");

    let block = client.get_block(&block_id).await?;
    
    println!("{block:#?}");

    Ok(block)
}

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses environment variables
    dotenvy::dotenv().ok();

    // Ensure envars are set
    for var in ["NODE_URL", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let node_url = std::env::var("NODE_URL").unwrap();
    let explorer_url = std::env::var("EXPLORER_URL").unwrap();
    let iota_block_tag = std::env::var("IOTA_BLOCK_TAG").unwrap();
    let iota_block_data = std::env::var("IOTA_BLOCK_DATA").unwrap();

    let client = connect_to_client_node(&node_url).await?;
    let block = add_tagged_data_block(&client, &iota_block_tag, &iota_block_data, &explorer_url).await?;
    retrieve_block_by_id(&client, &block.id()).await?;

    Ok(())
}
