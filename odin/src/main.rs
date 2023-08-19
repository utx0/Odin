use thorchain_rs::apis::configuration::Configuration;
use thorchain_rs::apis::vaults_api::asgard;
use thorchain_rs::apis::vaults_api::AsgardParams;

#[tokio::main]
async fn main() {
    let params = AsgardParams::default();
    let config = Configuration::default();

    let vaults = asgard(&config, params).await.unwrap();

    println!("VAULTS: {:#?}", vaults);
}
