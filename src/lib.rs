pub mod libs;
pub mod models;
pub mod utils;
pub mod server;
pub mod watcher;

pub use server::*;
pub use watcher::*;

use std::error::Error;

pub fn generate_all() -> Result<bool, Box<dyn Error>> {
    libs::build_all(None, None)
}
