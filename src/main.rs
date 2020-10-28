#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

mod helper;
use helper::path_exists;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::SystemTime;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Measurement {
    v: i8,       // version number, should be 1 here
    mac: String, // device mac adress
    co2: i16,    // co2 value in ppm
    temp: i8,    // temperature value in degrees celsius
    state: i8,   // device info
}

#[post("/measurement", format = "json", data = "<input>")]
fn post_new_measurement(input: Json<Measurement>) -> JsonValue {
    // mac adresses are usually 17 characters long, skip everything else
    if input.mac.len() == 17 {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let file_path = format!("./measurement/{}.csv", input.mac);

        let file = OpenOptions::new()
            .create_new(!path_exists(file_path.as_str()))
            .append(true)
            .open(file_path);

        if file.is_err() {
            return json!({
                "status": "error",
                "reason": "opening file"
            });
        }
        let res = writeln!(
            file.unwrap(),
            "{},{},{},{}",
            now.as_secs(),
            input.co2,
            input.temp,
            input.state
        );
        if res.is_err() {
            return json!({
                "status": "error",
                "reason": "writing file"
            });
        }
        return json!({ "status": "ok" });
    } else {
        json!({
            "status": "error",
            "reason": "invalid mac adress"
        })
    }
}

#[get("/measurements/<devicemac>")]
fn get_measurements(devicemac: &RawStr) -> JsonValue {
    if devicemac.len() != 17 {
        return json!({
            "status": "error",
            "reason": "not a valid mac address",
        });
    }
    let file_path = format!("./measurement/{}.csv", devicemac);
    let file = OpenOptions::new().read(true).open(file_path);
    if file.is_err() {
        return json!({
            "status": "error",
            "reason": "opening file"
        });
    }
    let mut csv_reader = csv::Reader::from_reader(file.unwrap());
    let mut timestamps: Vec<i64> = vec![];
    let mut co2_values: Vec<i16> = vec![];
    for record in csv_reader.records() {
        match record {
            Err(e) => {
                return json!({
                    "status": "error",
                    "reason": format!("{:?}", e)
                })
            }
            Ok(row) => {
                timestamps.push(row[0].parse::<i64>().unwrap());
                co2_values.push(row[1].parse::<i16>().unwrap());
            }
        }
    }

    return json!({
        "timestamps": timestamps,
        "co2_values": co2_values
    });
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/v1", routes![post_new_measurement, get_measurements])
        .mount("/", StaticFiles::from("./static"))
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
