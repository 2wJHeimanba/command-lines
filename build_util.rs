use chrono::{DateTime, Local};
use std::ops::Deref;
use std::path::Path;
use std::{fmt::Debug, io::Write};

pub fn log<T: Debug>(content: T) {
  let mut current_dir = if let Ok(res) = std::env::current_dir() {
    res
  } else {
    panic!("获取项目路径失败")
  };

  current_dir.push("println.log");

  let mut file = if let Ok(f) = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&current_dir)
  {
    f
  } else {
    panic!("文件操作失败")
  };

  let local_now: DateTime<Local> = Local::now();
  let local_now = local_now.format("%Y-%m-%d %H:%M:%S").to_string();

  let _ = file.write_fmt(format_args!("{local_now}: {content:#?}\n"));
}

// struct CustomError(&'static str);
// impl Display for CustomError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{}", self.0)
//   }
// }
// impl Debug for CustomError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{}", self.0)
//   }
// }
// impl Error for CustomError {}

pub fn copy_dir<T>(
  origin_path: T,
  destination_path: T,
) -> Result<(), Box<dyn std::error::Error>>
where
  T: Deref<Target = Path>,
{
  let origin = &*origin_path;
  let dest = &*destination_path;

  if !origin.is_dir() {
    log("输入的源目录路径不是目录文件");
    return Err("fdsafdsa".into());
  }

  if !destination_path.exists() && std::fs::create_dir(dest).is_err() {
    log("创建目录文件失败");
    return Err("".into());
  }

  if let Ok(val) = origin.read_dir() {
    for item in val {
      if item.is_err() {
        log("损失一个文件");
        continue;
      }

      let item = item.unwrap();
      let f_name = item.file_name();
      let mut temp_dest_dir = dest.to_path_buf();
      temp_dest_dir.push(f_name.to_str().unwrap());

      let _ = std::fs::copy(item.path(), temp_dest_dir);
    }
  }

  Ok(())
}
