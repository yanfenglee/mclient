use mclient::{get, post};
use reqwest::{Error};
use serde::{Serialize, Deserialize};

mod support;

use support::*;
use http::Method;
use url::Url;


#[derive(Deserialize, Serialize, Debug)]
struct ResA {
    #[serde(rename = "origin")]
    origin: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Login {
    id: i32,
    name: String,
    password: String,
}

#[tokio::test]
async fn get_with_query_param2() -> Result<(), Error> {
    let _server = server::http(31431, move |req| async move {
        assert_eq!(req.uri().path_and_query().unwrap().as_str(), "/path/sub?param1=1&param2=2&param3=hello");
        assert_eq!(req.headers().get("token").unwrap(), "xxx");

        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });

    #[get("http://127.0.0.1:31431/path/sub")]
    async fn get_test2(#[param("param1")]param1: i32, #[param]param2: i64, #[param]param3: String, #[header]token: String) -> Result<Login, Error> {}

    let res = get_test2(1, 2, "hello".to_string(), "xxx".to_string()).await?;
    assert_eq!(res.id, 100);

    Ok(())
}

#[tokio::test]
async fn test_get_with_body() -> Result<(), Error> {
    let _server = server::http(31420, move |_req| async move {
        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });


    #[get("http://127.0.0.1:31420/login")]
    async fn login(#[body]login: &Login) -> Result<Login, Error> {}

    let res = login(&Login {
        id: 0,
        name: "lyf".to_string(),
        password: "123456".to_string(),
    }).await?;

    assert_eq!(res.id, 100);

    #[get("http://127.0.0.1:31420/login")]
    async fn login2(#[body]login: &Login) -> Result<String, Error> {}

    let res = login2(&Login {
        id: 0,
        name: "lyf".to_string(),
        password: "123456".to_string(),
    }).await?;

    assert_eq!(res, r#"{"id":100,"name":"","password":""}"#);

    Ok(())
}

#[tokio::test]
async fn test_post_body() -> Result<(), Error> {
    let _server = server::http(31418, move |_req| async move {
        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });


    #[post("http://127.0.0.1:31418/login")]
    async fn login(#[body]login: &Login) -> Result<Login, Error> {}

    let res = login(&Login {
        id: 0,
        name: "lyf".to_string(),
        password: "123456".to_string(),
    }).await?;

    assert_eq!(res.id, 100);

    #[post("http://127.0.0.1:31418/login")]
    async fn login2(#[body]login: &Login) -> Result<String, Error> {}

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

#[tokio::test]
async fn test_request_raw() -> Result<(), Error> {
    let server = server::http(0, move |_req| async move {
        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });

    let url = format!("http://{}/ua", server.addr());
    let mut reqb = reqwest::Client::builder()
        .build()
        .expect("client builder")
        .request(Method::GET, Url::parse(url.as_str()).unwrap());

    reqb = reqb.header("token", "tokenxxx");
    reqb = reqb.query(&[("a", 3), ("b", 4), ]);

    let res: Login = reqb
        .json(&Login::default())
        .send()
        .await?
        .json()
        .await?;

    assert_eq!(res.id, 100);

    Ok(())
}

#[tokio::test]
async fn test_path_variable() -> Result<(), Error> {
    let _server = server::http(31419, move |req| async move {
        assert_eq!(req.uri().path(), "/user/worker/123");
        assert_eq!(req.headers().get("token").unwrap(), "xxx");

        let login = Login { id: 100, name: "".to_string(), password: "".to_string() };

        http::Response::new(serde_json::to_string(&login).unwrap().into())
    });

    #[get("http://127.0.0.1:31419/user/{type}/{id}")]
    async fn get_path_variable(#[path("type")]type1: String, #[path]id: i32, #[header]token: String) -> Result<Login, Error> {}

    let res = get_path_variable("worker".to_string(), 123, "xxx".to_string()).await?;
    assert_eq!(res.id, 100);

    Ok(())
}

// #[tokio::test]
// async fn test_generate() -> Result<(), Error> {
//     async fn path_test(name1: String, value: i32) -> Result<String, Error>
//     {
//         use std::str::FromStr;
//         use http::Method;
//         use url::Url;
//         use std::collections::HashMap;
//         use mclient;
//
//         let url = format!("{}", "https://httpbin.org/cookies/set/{name}/{value}");
//         let mut path_variables: HashMap<&str, String> = HashMap::new();
//         path_variables.insert("name", format!("{}", name1));
//         path_variables.insert("value", format!("{}", value));
//
//         let url = mclient::str_utils::replace_named(url.as_str(), &path_variables);
//         println!("url: {}", url);
//
//         let client = reqwest::Client::new();
//         let method = Method::from_str("GET").unwrap();
//         let mut reqb = client.request(method, Url::parse(url.as_str()).unwrap());
//         let resp: Result<String, Error> = reqb.send().await?.text().await;
//
//         resp
//     }
//
//     let _res = path_test("asdf".to_string(), 123).await?;
//     println!("{}", res);
//     Ok(())
// }