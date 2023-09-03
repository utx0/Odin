use state::State;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

mod state;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(state::State::new()));

    let (_tx, rx) = oneshot::channel::<()>();

    spawn_churn_checks(state.clone()).await;

    println!("Obin is watching...");

    let _ = rx.await;
}

async fn spawn_churn_checks(state: Arc<Mutex<State>>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let mut state = state.lock().await;
            state.update_churn_interval().await;
            state.update_vaults().await;
            let seconds = state.seconds_to_churn().await;
            println!("{} seconds before next churn checks", seconds);
            tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        }
    })
}
