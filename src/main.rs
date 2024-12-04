use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://test-dashboard.kylix.finance").await?;
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;
    
    while let Some(block) = blocks_sub.next().await {
        let new_block = block?; 
        let block_number = new_block.header().number;
        
        println!("New block #{block_number} created! ✨");

        let extrinsics = new_block.extrinsics().await?;

        for extrinsic in extrinsics.iter() {
            match extrinsic {
                Ok(extrinsic_details) => {
                    let events = extrinsic_details.events().await?;
                
                    for evt in events.iter() {
                        // try to parse the current event into a Transfer Event
                        let parsed_transfer = evt?.as_event::<polkadot::balances::events::Transfer>()?;
                        
                        // check if we have some valid transfer 
                        match parsed_transfer {
                            Some(transfer) => println!("
                                {:?} transfered {:?} to {:?}",  transfer.from.to_string(), transfer.amount, transfer.to.to_string()),
                            _ => println!("No transfer events in this block") 
                        }
                    }
                },
                Err(e) => {
                    println!("Encountered an error: {}", e);
                },
            }
        }
    }

    Ok(())
}