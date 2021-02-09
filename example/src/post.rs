use mclient::{post, Error};
use serde::{Serialize};

#[derive(Serialize, Debug)]
struct PostData {
    name: String,
    permissions: Vec<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    #[post("https://httpbin.org/post")]
    async fn post_data(#[body]data: &PostData) -> Result<String, Error> {}

    let data = PostData {
        name: "jjj".to_string(),
        permissions: vec![100,200,300],
    };

    let res = post_data(&data).await?;

    println!("res: {:?}", res);

    Ok(())
}