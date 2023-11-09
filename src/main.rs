use bdk:: {Wallet, wallet::AddressIndex};
use bdk::database::MemoryDatabase;
use bdk::blockchain::ElectrumBlockchain;
use bdk::SyncOptions;
use bdk::electrum_client::Client;
use bdk::bitcoin::Network;
use bdk::wallet::AddressInfo; 

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Connect to electrum server 
    let blockchain = ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?); 
    // Create a new wallet
    let wallet = Wallet::new(
        "wpkh([3a67b6ec/84h/1h/0h]tprv8ZgxMBicQKsPe1eUZwjZyCwPsgVNfdhM1vwi7DA5CRXxg6kNLTtMPtQP4xYcsm5PBzBqDfp4AAsZ9wHK7oe7gEePKRrXXFjPckaY3uzczsC/0/*)",
        Some("wpkh([3a67b6ec/84h/1h/0h]tprv8ZgxMBicQKsPe1eUZwjZyCwPsgVNfdhM1vwi7DA5CRXxg6kNLTtMPtQP4xYcsm5PBzBqDfp4AAsZ9wHK7oe7gEePKRrXXFjPckaY3uzczsC/1/*)"),
        Network::Testnet, // Use testnet
        MemoryDatabase::default(), // Use memory database for now
    )?;

    let addr: AddressInfo = wallet.get_address(AddressIndex::New)?;
    println!("Address: {}", addr);

    wallet.sync(&blockchain, SyncOptions::default())?; // Sync wallet

    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);

    Ok(())
}