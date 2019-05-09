use crate::CONFIG;
use rocket::response::status;

#[put("/<domain>")]
fn add_entry(domain: String) -> Result<status::Accepted<String>, rusqlite::Error> {
    CONFIG.get().insert_entry(&domain)?;
    Ok(status::Accepted(Some(format!("Added domain {}", domain))))
}

#[delete("/<domain>")]
fn delete_entry(domain: String) -> Result<status::Accepted<String>, rusqlite::Error> {
    CONFIG.get().remove_entry(&domain)?;
    Ok(status::Accepted(Some(format!("Removed domain {}", domain))))
}

#[get("/")]
fn show_all() -> Option<String> {
    match CONFIG.get().get_entry() {
        Ok(entries) => {
            let mut output: String = String::new();
            for entry in entries {
                output.push_str(&format!("{}\n", entry));
            }
            Some(output)
        }
        Err(_) => None,
    }
}

pub fn run_server() {
    rocket::ignite()
        .mount("/", routes![add_entry, delete_entry, show_all])
        .launch();
}
