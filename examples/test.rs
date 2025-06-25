use std::fmt::Display;

trait People {
  type Gender: Display;
  // type Age = i32;
  const LANGUAGE: &'static str = "wenjian";
  const NAME: i32;

  fn new() -> Self;
  fn get_name(&self);

  fn is_people() -> bool {
    Self::LANGUAGE;
    true
  }
}

trait People2 {
  fn is_people() -> bool {
    false
  }
}
// const name: String = String::from("fdsafds");
struct Person;
impl People2 for Person {}
impl People for Person {
  const NAME: i32 = 23;
  type Gender = String;
  fn new() -> Self {
    Person
  }
  fn get_name(&self) {
    println!("person");
  }
}

fn main() {
  // let test = People::is_people();
  let person = Person;
  let res = Person::new();
  let bo = <Person as People>::is_people();
  if bo {
    println!("是真的");
  } else {
    println!("不是真的");
  }
  println!("fdsafds");
}
