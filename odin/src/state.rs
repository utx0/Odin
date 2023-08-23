use thorchain_rs::apis::configuration::Configuration;
use thorchain_rs::apis::mimir_api;
use thorchain_rs::apis::mimir_api::MimirKeyParams;
use thorchain_rs::apis::vaults_api::{asgard, AsgardParams};

const CHURN_MAX_RANGE: i64 = 43200;

pub struct State {
    churn_interval: Option<i64>,
    // vaults: Vec<Vault::vault>,
    vaults: Vec<thorchain_rs::models::vault::Vault>,
    config: Configuration,
}

impl State {
    pub fn new() -> Self {
        Self {
            churn_interval: None,
            vaults: Vec::new(),
            config: Configuration::default(),
        }
    }

    pub async fn update_churn_interval(&mut self) {
        let params = MimirKeyParams {
            key: "CHURNINTERVAL".to_string(),
            height: None,
        };

        let churn_interval = mimir_api::mimir_key(&self.config, params).await.unwrap();

        self.churn_interval = Some(churn_interval);
    }

    pub fn churn_interval(&self) -> Option<i64> {
        self.churn_interval
    }

    pub async fn update_vaults(&mut self) {
        let asgard_params = AsgardParams::default();

        let vaults = asgard(&self.config, asgard_params).await.unwrap();

        self.vaults = vaults;
    }

    pub fn vaults(&self) -> Vec<thorchain_rs::models::vault::Vault> {
        self.vaults.clone()
    }

    pub fn print_vaults(&self) {
        println!("{:#?}", self.vaults);
    }
}
