#![feature(const_fn, proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rusqlite;

use state::LocalStorage;

pub static CONFIG: LocalStorage<blackrs::DBC> = LocalStorage::new();

mod server;

fn main() -> Result<(), Box<std::error::Error>> {
    CONFIG.set(|| blackrs::DBC::new("/tmp/blacklist.db"));

    server::run_server();

    Ok(())
}
