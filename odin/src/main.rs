use state::State;
use thorchain_rs::apis::configuration::Configuration;
use thorchain_rs::apis::vaults_api::asgard;
use thorchain_rs::apis::vaults_api::AsgardParams;

mod state;

#[tokio::main]
async fn main() {
    let mut state = state::State::new();

    state.update_vaults().await;

    println!("{:?}", state.print_vaults()); 
}
