mod editor;
mod html;
mod log;
mod macro_core;
mod server;
mod service;
pub mod tools;

pub use editor::*;
pub use html::*;
pub use log::*;
pub use macro_core::*;
pub use server::*;
pub use service::*;
// pub use tools;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn test() {
  rk!()
}
