mod editor;
mod html;
mod macro_core;
mod server;
mod service;

pub use editor::*;
pub use html::*;
pub use macro_core::*;
pub use server::*;
pub use service::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn test() {
  rk!()
}
