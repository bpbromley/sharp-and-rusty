#[macro_use] extern crate rocket;
use std::collections::HashMap;
use std::sync::Mutex;
use rocket::data::{Data, ToByteUnit};
use rocket::State;

struct Cache {
    data: Mutex<HashMap<String, String>>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/cache/<key>")]
async fn cache_read(state: &State<Cache>, key: &str) -> String {
    let lock = state.data.lock().expect("lock shared data");
    match lock.contains_key(key) {
        true => {
            let opt = lock.get(key);
            let value = match opt {
                Some(value) => value,
                None => ""
            };
            value.to_string()
        },
        false => {
            "".to_string()
        }
    }
}

#[get("/keys")]
fn cache_list_keys(state: &State<Cache>) -> String {
    let lock = state.data.lock().expect("lock shared data");
    let key_list = lock.keys().fold("".to_string(), |acc, key| {
        acc + key + ","
    });
    key_list
}

#[get("/health")]
fn health() -> String {
    "ok".to_string()
}

#[post("/cache/<key>", data = "<data>")]
async fn cache_write(state: &State<Cache>, key: &str, data: Data<'_>) -> std::io::Result<()> {
    let val = data.open((512 as u16).kibibytes())
        .into_string()
        .await?;
    let mut lock = state.data.lock().expect("lock shared data");
    lock.insert(key.to_string(), val.to_string());
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, cache_write, cache_read, cache_list_keys, health])
        .manage(Cache { data: Mutex::new(HashMap::new()) })
}
