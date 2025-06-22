mod build_util;
use build_util::{copy_dir, log};

// use std::{env::current_dir, io::{Read, Write}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let current_exe = if let Ok(mut f) = std::env::current_dir() {
    f.push("static");
    f
  } else {
    log("没有任何内容");
    panic!("f");
  };

  let dest_path = if let Ok(f) = std::env::current_exe() {
    let mut temp_path = f
      .parent()
      .unwrap()
      .to_path_buf()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .to_owned();

    let profile = std::env::var("PROFILE").unwrap();
    temp_path.push(profile);
    temp_path.push("static");
    temp_path
    // let res = temp_path.parent().unwrap().parent().unwrap().to_owned();
    // log(temp_path.to_str().unwrap());
    // f.pop();
    // f.push("static");
    // f
  } else {
    log("目标路径获取失败");
    panic!()
  };

  if copy_dir(current_exe, dest_path).is_ok() {
    log("good: 目录文件复制成功");
  } else {
    log("bad: 文件转移失败！");
  }

  // log(&current_exe);

  // let mut cpath = current_dir()?;

  // cpath.push("index.txt");

  // if cpath.exists() {
  //     let mut content = Vec::new();
  //     let mut file = std::fs::OpenOptions::new().create(true).read(true).write(true).open(cpath)?;
  //     file.read_to_end(&mut content)?;
  //     content.push(58);
  //     file.write_all(&content);
  // }else{
  //     let mut file = std::fs::OpenOptions::new().create(true).write(true).open(cpath)?;
  //     file.write_all(b"hello world")?;
  // }

  println!("cargo:rerun-if-changed=build.rs");

  Ok(())
}
