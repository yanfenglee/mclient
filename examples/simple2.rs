use mclient::{get};
use reqwest::{Error};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct IP {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    #[get("https://httpbin.org/ip")]
    async fn get_ip() -> Result<IP, Error> {}

    let res = get_ip().await?;

    println!("res: {:?}", res);

    Ok(())
}