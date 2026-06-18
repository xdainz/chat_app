use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    age: u32,
}

fn main() {
    let user = User {
        username: "dainz".to_owned(),
        age: 22,
    };

    let serialized = serde_json::to_string(&user).unwrap();

    println!("serialized = {}", serialized)
}
