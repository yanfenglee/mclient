use mclient::{mc, get, Error};

#[mc("https://httpbin.org")]
mod car {
    #[get("/ip")]
    pub async fn get_ip(#[param]param: i32) -> Result<String, super::Error> {}
}

#[tokio::test]
async fn test1() {
    let res = car::get_ip(1).await;
    println!("res: {:?}", res);
}

