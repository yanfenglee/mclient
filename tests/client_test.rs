use mclient::{get, post};
use reqwest::{Error};
use serde::{Serialize, Deserialize};

mod support;

use support::*;


#[derive(Deserialize, Serialize, Debug)]
struct ResA {
    origin: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Login {
    id: i32,
    name: String,
    password: String,
}

#[tokio::test]
async fn get_with_query_param() -> Result<(), Error> {
    let _server = server::http(31417, move |req| async move {
        assert_eq!(req.uri().path_and_query().unwrap().as_str(), "/path/sub?param1=1&param2=2&param3=hello");

        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });

    #[get("http://127.0.0.1:31417/path/sub")]
    async fn get_test(param1: i32, param2: i64, param3: String) -> Result<String, Error> {}

    let res = get_test(1, 2, "hello".to_string()).await?;

    assert_eq!(r#"{"id":100,"name":"","password":""}"#, res);

    #[get("http://127.0.0.1:31417/path/sub")]
    async fn get_test2(param1: i32, param2: i64, param3: String) -> Result<Login, Error> {}

    let res = get_test2(1, 2, "hello".to_string()).await?;
    assert_eq!(res.id, 100);

    Ok(())
}

#[tokio::test]
async fn test_post_body() -> Result<(), Error> {
    let _server = server::http(31418, move |_req| async move {
        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });


    #[post("http://127.0.0.1:31418/login")]
    async fn login(login: &Login) -> Result<Login, Error> {}

    let res = login(&Login {
        id: 0,
        name: "lyf".to_string(),
        password: "123456".to_string(),
    }).await?;

    assert_eq!(res.id, 100);

    #[post("http://127.0.0.1:31418/login")]
    async fn login2(login: &Login) -> Result<String, Error> {}

    let res = login2(&Login {
        id: 0,
        name: "lyf".to_string(),
        password: "123456".to_string(),
    }).await?;

    assert_eq!(res, r#"{"id":100,"name":"","password":""}"#);

    Ok(())
}


#[tokio::test]
async fn test2() -> Result<(), Error> {
    println!("name = {}", stringify!(hget));
    let path = format!("{}", "https://httpbin.org/ip");
    println!("path: {}", path);
    let client = reqwest::Client::new();
    let mut reqb = client.get(&path);
    reqb = reqb.header("token", "tokenxxx");
    reqb = reqb.query(&[("a", 3), ("b", 4), ]);
    let resp: Result<ResA, Error> = reqb
        .send().await?.json().await;

    println!("{:#?}", resp);

    Ok(())
}

#[tokio::test]
async fn test4() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let host = "https://httpbin.org/ip";

    use reqwest::header;
    let mut headers = header::HeaderMap::new();
    let token = "asdfasdf";
    headers.insert("token", token.parse().unwrap());

    let mut reqb = client.get(host);
    reqb = reqb.header("token", "tokenxxx");
    reqb = reqb.query(&[("a", 3), ("b", 4), ]);

    let resp: ResA = reqb.send().await?.json().await?;

    let res = format!("{:#?}", resp);
    println!("{}", res);
    Ok(())
}

#[tokio::test]
async fn user_agent() {
    let server = server::http(0, move |req| async move {
        assert_eq!(req.headers()["user-agent"], "reqwest-test-agent");
        http::Response::default()
    });

    let url = format!("http://{}/ua", server.addr());
    let res = reqwest::Client::builder()
        .user_agent("reqwest-test-agent")
        .build()
        .expect("client builder")
        .get(&url)
        .send()
        .await
        .expect("request");

    assert_eq!(res.status(), reqwest::StatusCode::OK);
}


#[tokio::test]
async fn test_post() -> Result<(), Error> {
    let server = server::http(0, move |_req| async move {
        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });

    let url = format!("http://{}/ua", server.addr());
    let client = reqwest::Client::builder()
        .build()
        .expect("client builder")
        .post(&url);

    let res: Login = client
        .json(&Login::default())
        .send()
        .await?
        .json()
        .await?;

    assert_eq!(res.id, 100);

    Ok(())
}
