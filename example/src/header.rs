use mclient::{get, Error};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct IP {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    #[get("https://httpbin.org/ip")]
    async fn get_ip(#[param]name: String, #[param("sex")]gender: i32, #[header]token: String) -> Result<IP, Error> {}

    let res = get_ip("mclient".to_string(), 1,"xxxx".to_string()).await?;

    println!("res: {:?}", res);

    Ok(())
}