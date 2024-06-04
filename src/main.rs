mod errors;

use balter::prelude::*;
// use errors::LoadTestingError;

#[allow(unused_imports)]
use reqwest::{Client, Error, Response};
use std::sync::OnceLock;
use std::time::Duration;

static CLIENT: OnceLock<Client> = OnceLock::new();
const HEALTH_URL: &str = "http://localhost:3000/api/health";
const ACCOUNTS_URL: &str = "http://localhost:3000/accounts/2";
const ACCOUNTS_SIWF_URL: &str = "http://localhost:3000/accounts/siwf";

use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Run a scenario in parallel for 3600s such that:
    // - Max 5,000 transactions are sent per second
    // - Max p95 latency is 100ms
    // - Max error rate is 3%
    FmtSubscriber::builder()
        .with_env_filter("balter=debug")
        .init();

    let _stats = check_health_scenario()
        .tps(5_000)
        .duration(Duration::from_secs(120))
        .await;
}

// A Scenario is just an async Rust function, and
// can contain any complex logic you need.
#[scenario]
async fn check_health_scenario() {
    let _ = check_health().await;
    let _ = get_accounts().await;
    let _ = post_accounts_siwf().await;
}

// A Transaction is also just an async Rust function, and
// provides flexibility with what you want Balter to measure
// and constrain on.
#[transaction]
async fn check_health() -> Result<(), Error> {
    // GET /health
    let client = CLIENT.get_or_init(Client::new);
    let response = client.get(HEALTH_URL).send().await?;

    match response.status().as_u16() {
        200..=299 => {
            let _body = response.text().await?;
            // println!("Health check passed: {}", body);
            // println!("Health check passed");
        }
        400..=599 => {
            let status = response.status();
            let error_message = response.text().await?;
            println!("Error {}: {}", status, error_message);
        }
        _ => {
            println!("Unexpected status code: {}", response.status());
        }
    }
    Ok(())
}

#[transaction]
async fn get_accounts() -> Result<(), Error> {
    let client = CLIENT.get_or_init(Client::new);
    // let response = client.get(format!("{}/2", ACCOUNTS_URL)).send().await?;
    let response = client.get(ACCOUNTS_URL).send().await?;
    // REMOVE: 
    // println!("response: {:?}", response.status());

    match response.error_for_status() {
        Ok(_) => {
            // println!("Accounts check passed");
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[transaction]
async fn post_accounts_siwf() -> Result<(), Error> {
    let client = CLIENT.get_or_init(Client::new);
    let response = client.post(ACCOUNTS_SIWF_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "signUp": {
              "extrinsics": [
                {
                  "pallet": "msa",
                  "extrinsicName": "createSponsoredAccountWithDelegation",
                  "encodedExtrinsic": "0xed01043c01b01b4dcafc8a8e73bff98e7558249f53cd0e0e64fa6b8f0159f0913d4874d9360176644186458bad3b00bbd0ac21e6c9bd5a8bed9ced7a772d11a9aac025b47f6559468808e272696f596a02af230951861027c0dc30f7163ecf316838a0723483010000000000000014000000000000000000004d000000"
                },
                {
                  "pallet": "handles",
                  "extrinsicName": "claimHandle",
                  "encodedExtrinsic": "0xb901044200b01b4dcafc8a8e73bff98e7558249f53cd0e0e64fa6b8f0159f0913d4874d93601225508ae2da9804c60660a150277eb32b2a0f6b9c8f6e07dd6cad799cb31ae1dfb43896f488e9c0b7ec8b530d930b3f9b690683f2765d5def3fee3fc6540d58714656e6464794d000000"
                }
              ]
            }
          }))
        .send()
        .await?;
    // REMOVE: 
    // println!("response: {:?}", response.status());

    match response.error_for_status() {
        Ok(_) => {
            // println!("SIWF check passed");
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[transaction]
async fn post_accounts_siwf_sign_in() -> Result<(), Error> {
    let client = CLIENT.get_or_init(Client::new);
    let response = client.post(ACCOUNTS_SIWF_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "signIn": {
              "siwsPayload": {
                "message": "localhost wants you to sign in with your Frequency account:\n5Fghb4Wt3sg9cF6Q2Qucp5jXV5pL2U9uaYXwR9R8W8SYe9np\n\nThe domain localhost wants you to sign in with your Frequency account via localhost\n\nURI: http://localhost:5173/signin/confirm\nNonce: N6rLwqyz34oUxJEXJ\nIssued At: 2024-03-05T23:18:03.041Z\nExpiration Time: 2024-03-05T23:23:03.041Z",
                "signature": "0x38faa2fc6f59bef8ffccfc929fb966e1d53ba45e3af7a029ea1d636eaddcbe78a4be0f89eaf7ff7bbaef20a070ad65f9d0f876889686687ef623214fddddb18b"
              }
            }
          }))
        .send()
        .await?;
    // REMOVE: 
    // println!("response: {:?}", response.status());

    match response.error_for_status() {
        Ok(_) => {
            // println!("SIWF check passed");
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}
