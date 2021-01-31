# mclient
macro based http client for rust

### examples
```rust
use mclient::{post};
use reqwest::{Error};
use serde::{Serialize};

#[derive(Serialize, Debug)]
struct PostData {
    name: String,
    permissions: Vec<i32>,
}

#[derive(Deserialize, Debug)]
struct IP {
    origin: String,
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

    #[get("https://httpbin.org/ip")]
    async fn get_ip(#[param]name: String, #[param("sex")]gender: i32, #[header]token: String) -> Result<IP, Error> {}

    let res = get_ip("mclient".to_string(), 1,"xxxx".to_string()).await?;

    println!("res: {:?}", res);

    #[get("https://httpbin.org/cookies/set/{name}/{value}")]
    async fn path_test(#[path("name")]name1: String, #[path]value: i32) -> Result<String, Error> {}

    let res = path_test("num".to_string(), 111).await?;

    println!("res: {:?}", res);


    Ok(())
}
```

## License

This project is licensed under the [MIT license](LICENSE).
