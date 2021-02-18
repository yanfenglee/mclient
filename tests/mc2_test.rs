use mclient::{mc2, get, Error};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct IP {
    origin: String,
}

struct Host;

#[mc2("https://httpbin.org")]
impl Host {
    #[get("/ip")]
    pub async fn get_ip(#[param]param: i32) -> Result<String, Error> {}

    #[get("/ip")]
    pub async fn get_ip2(#[param]param: i32) -> Result<String, Error> {}
}

#[tokio::test]
async fn test1() {
    let res = Host::get_ip(1).await;
    assert!(res.is_ok());

    let res = Host::get_ip2(1).await;
    assert!(res.is_ok());
}

