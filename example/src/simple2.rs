use mclient::{mclient, Error};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct IP {
    origin: String,
}

struct Host;

#[mclient("https://httpbin.org")]
impl Host {
    #[get("/ip")]
    pub async fn get_ip(#[param]param: i32) -> Result<String, Error> {}

    #[get("/ip")]
    pub async fn get_ip2(#[param]param: i32) -> Result<IP, Error> {}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let res = Host::get_ip(1).await;
    println!("res: {:?}", res);

    let res = Host::get_ip2(1).await;
    println!("res2: {:?}", res);

    Ok(())
}

