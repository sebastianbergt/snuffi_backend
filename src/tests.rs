use rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn post_measurement() {
    let client = Client::new(rocket()).unwrap();

    let json_measurement =
        r#"{"v": 1, "mac": "01:02:03:04:05:06", "co2": 450, "temp": 20, "state": 0}"#;

    // Add a new measurement
    let res = client
        .post("/v1/measurement")
        .header(ContentType::JSON)
        .body(json_measurement)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);
}
