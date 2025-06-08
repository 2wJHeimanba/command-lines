mod macro_core;
mod editor;
pub use macro_core::*;
pub use editor::*;


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn test() {
  rk!()
}