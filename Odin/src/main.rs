extern crate dotenv;

use dotenv::dotenv;
use std::env;

const NODE_URI: &'static str = "/thorchain/nodes";
const VAULTS_URI: &'static str = "/thorchain/vaults/asgard";

fn main() {
    dotenv().ok();

    let node = env::var("NODE").expect("NODE must be set");

    println!("NODE_URL: {}{}", node, NODE_URI);
    println!("VAULTS_URL: {}{}", node, VAULTS_URI);

    let node_url = format!("{}{}", node, NODE_URI);
    let vaults_url = format!("{}{}", node, VAULTS_URI);

    let resp = reqwest::blocking::get(&node_url).unwrap();
    let vaults = reqwest::blocking::get(&vaults_url).unwrap();

    println!("NODE: {}", resp.status());
}
