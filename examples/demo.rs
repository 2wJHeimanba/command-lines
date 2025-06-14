use command_lines::Result;

fn main() -> Result<()> {
  // let mut editor = Editor::new()?;
  // editor.run();
  // println!("fdsafdsafds");
  // #[allow(clippy::useless_vec)]
  let mut v = vec![1, 2, 3];

  #[allow(clippy::disallowed_methods)]
  std::vec::Vec::push(&mut v, 23);

  println!("v: {}", v[3]);

  let mut res = None;
  let _ = res.insert(223);

  println!("res: {}", res.unwrap());

  let name = &String::from("hello world");
  println!("{name}");

  Ok(())
}
