use nifi_rs::apis::access_api::CreateAccessTokenError;
use nifi_rs::apis::configuration::Configuration;
use nifi_rs::apis::Error;
use nifi_rs::apis::flow_api::SearchFlowError;
use nifi_rs::models::SearchResultsEntity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("test");
    let mut config = Configuration::new();
    let token = nifi_rs::apis::access_api::create_access_token(&config, Some("admin"), Some("supersecretpassword")).await;
    match token {
        Ok(token) => config.bearer_access_token = Some(token),
        Err(er) => println!("got error {:?} ", er),
    }
    println!("{:?}", config);
    let processors = nifi_rs::apis::flow_api::search_flow(&config, Some("stackable"), None).await;
    match processors {
        Ok(proces) => println!("OK\n{:?}", proces),
        Err(err) => println!("ERR\n{:?}", err),
    }
    Ok(())
}