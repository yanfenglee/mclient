# mclient
macro based http client for rust

### examples
```rust
#[post("http://127.0.0.1:31416/login")]
async fn login(login: &Login) -> Result<Login, Error> {}

let res = login(&Login{
    id: 0,
    name: "lyf".to_string(),
    password: "123456".to_string()
}).await?;
```
