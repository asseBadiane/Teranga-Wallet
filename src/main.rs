use bdk:: {Wallet, wallet::AddressIndex, bitcoin::Address};
use bdk::database::MemoryDatabase;
use bdk::blockchain::ElectrumBlockchain;
use bdk::SyncOptions;
use bdk::electrum_client::Client;
use bdk::bitcoin::Network;
use bdk::wallet::AddressInfo; 
use std::str::FromStr;
use bdk::SignOptions;
use bdk::blockchain::Blockchain;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Create a new wallet
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

    // Generate a new address for the wallet
    wallet.sync(&blockchain, SyncOptions::default())?; // Sync wallet

    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);

    
    // Create a transaction 
    let faucet_address = Address::from_str("tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6")?;
    let mut tx_builder = wallet.build_tx();
    tx_builder
    .add_recipient(faucet_address.script_pubkey(), balance.get_total() / 2)
    .enable_rbf();

    let (mut psbt, tx_details) = tx_builder.finish()?;

    println!("Transaction details: {:#?}", tx_details);

    // Sign the transaction
    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized, "Tx has not been finalized");
    println!("Transaction Signed: {}", finalized);

    // Broadcast the transaction 
    let raw_transaction = psbt.extract_tx();
    let txid = raw_transaction.txid();
    blockchain.broadcast(&raw_transaction)?;
    println!("Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}", txid = txid);

    Ok(())
}


// bitcoin: