use mclient::{mc, get, Error};

#[mc("http://localhost")]
mod Car {
    #[get("/path/sub")]
    async fn get_test2(#[param("param1")]param1: i32) -> Result<String, Error> {}
}


async fn
get_test2(param1: i32, param2: i64, param3: String, token: String) ->
Result<String, Error>
{
    use std::str::FromStr;
    use std::collections::HashMap;
    use
    mclient;
    let mut path_variables: HashMap<&str, String> = HashMap::
    new();
    let url = mclient::str_utils::
    replace_named("http://127.0.0.1:31431/path/sub", &path_variables);

    let client = mclient::Client::new();
    let method = mclient::Method::from_str("GET").unwrap();

    let urls = "http://127.0.0.1:31431/path/sub";
    println!("url is111 : {}", urls);
    let uurl = mclient::Url::parse(url.as_str()).unwrap();


    println!("url is222 : {}", uurl.clone());
    let mut reqb = client.request(method, uurl);


    reqb
        = reqb.header("token", token);
    reqb = reqb.
        query(&[("param1", param1), ]);
    reqb = reqb.
        query(&[("param2", param2), ]);
    reqb = reqb.
        query(&[("param3", param3), ]);
    let resp: Result<String, Error> = reqb
        .send().await?.json().await;
    resp
}

#[tokio::test]
async fn test1() {
    let res = get_test2(1, 2, "asdf".to_string(), "asdf".to_string()).await;
    println!("res: {:?}", res);
}

#[test]
fn test2() {
    let url = "http://127.0.0.1:31431/path/sub";
    let url = mclient::Url::parse(url).unwrap();
    println!("url: {:?}", url);
}
