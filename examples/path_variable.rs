use mclient::{get};
use reqwest::{Error};

#[tokio::main]
async fn main() -> Result<(), Error> {

    #[get("https://httpbin.org/cookies/set/{name}/{value}")]
    async fn path_test(#[path("name")]name1: String, #[path]value: i32) -> Result<String, Error> {}

    let res = path_test("num".to_string(), 111).await?;

    println!("res: {:?}", res);

    Ok(())
}