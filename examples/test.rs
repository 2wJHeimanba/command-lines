struct Age(u8);
impl TryFrom<i32> for Age {
  type Error = &'static str;
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    Ok(Self(value as u8))
  }
}

fn main() {
  let age: Age = 23i32.try_into().unwrap();
  let content = age.0;
  println!("content: {}", content);
}
