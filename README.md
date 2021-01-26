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

#[tokio::main]
async fn main() -> Result<(), Error> {

    #[post("https://httpbin.org/post")]
    async fn post_data(data: &PostData) -> Result<String, Error> {}

    let data = PostData {
        name: "jjj".to_string(),
        permissions: vec![100,200,300],
    };

    let res = post_data(&data).await?;

    println!("res: {:?}", res);

    Ok(())
}
```

## License

This project is licensed under the [MIT license](LICENSE).
