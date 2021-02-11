use mclient::{mc2, get, Error};

struct Car;

#[mc2("https://httpbin.org")]
impl Car {
    #[get("/ip")]
    async fn get_ip(#[param]param: i32) -> Result<String, Error> {}

    #[get("/ip")]
    async fn get_ip2(#[param]param: i32) -> Result<String, Error> {}
}

#[tokio::test]
async fn test1() {
    let res = Car::get_ip(1).await;
    println!("res: {:?}", res);

    let res = Car::get_ip2(1).await;
    println!("res: {:?}", res);
}

