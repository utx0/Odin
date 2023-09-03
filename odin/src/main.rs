use state::State;
use std::sync::Arc;
use thorchain_rs::apis::configuration::Configuration;
use thorchain_rs::apis::vaults_api::asgard;
use thorchain_rs::apis::vaults_api::AsgardParams;
use tokio::sync::Mutex;

mod state;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(state::State::new()));

    check_vaults(state.clone()).await;
}

async fn check_vaults(state: Arc<Mutex<State>>) {
    loop {
        let state = state.clone();
        let task = tokio::spawn(async move {
            let mut state = state.lock().await;
            state.update_vaults().await;
            println!("{:?}", state.print_vaults());
        });

        task.await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; //TODO create a smarter way to time the loop
    }
}
