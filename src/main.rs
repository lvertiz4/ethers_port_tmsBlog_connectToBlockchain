use ethers_core::{abi::Abi, utils::format_ether};
use ethers_contract::Contract;
use ethers::{prelude::*};
// use dotenv;
use std::{fs, env, sync::Arc};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let provider = Provider::<Http>::try_from(&env::var("ALCHEMY_GOERLI_URL").unwrap())?;

    let account:H160 = env::var("ACCOUNT_ADDRESS").unwrap().parse::<Address>()?;
   
    let balance = provider.get_balance(*&account, None).await?;
    println!("Current balance of Account {:?} is approximately {} Ether", &account, format_ether(&balance));
    
    let current_block = provider.get_block_number().await?;
    //U64 implements Display trait, so you can use the variable 'current_block" inside your println! macro
    println!("Got Block: {}", &current_block);


    // // Address is USDC goerli contract address
    let usdc_goerli_addr = "0x07865c6e87b9f70255377e024ace6630c1eaa37f".parse::<Address>()?;

    // let usdc_abi_json = File::open("./usdc_abi.json").unwrap();
    // let mut contents = String::new();
    let abi_string = fs::read_to_string("./usdc_abi.json")?;

    let abi: Abi = serde_json::from_str(&abi_string).unwrap();

    let contract = Contract::new(usdc_goerli_addr, abi, Arc::new(provider));

    let contract_name: String = contract.method::<_, String>("name", ())?.call().await?;
    let contract_total_supply: U256 = contract.method::<_, U256>("totalSupply", ())?.call().await?;

    println!("Contract name: {}, Total token supply minted by contract: {}", &contract_name, &contract_total_supply);

    Ok(())
}

// abigen!(
//     USDC_Goerli,
//     "usdc_abi.json",
//     event_derives(serde::Deserialize, serde_json::Serialize)
// )



// code above inspired by https://docs.rs/ethers/1.0.2/ethers/providers/index.html#examples
// further ported to match https://tms-dev-blog.com/rust-web3-connect-to-ethereum/