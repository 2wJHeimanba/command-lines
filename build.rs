use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut curr_workspace_path = std::env::current_dir()?;
  curr_workspace_path.push("static");

  let mut target_static_path: PathBuf = std::env::var("OUT_DIR")?.into();
  target_static_path.push("static");
  println!("target: {:?}", target_static_path);
  if !target_static_path.exists() {
    std::fs::create_dir(&target_static_path)?;
  }

  #[allow(clippy::collapsible_if)]
  if curr_workspace_path.exists() {
    if curr_workspace_path.is_dir() {
      let origin_entries = std::fs::read_dir(&curr_workspace_path)?;
      for file in origin_entries {
        if let Ok(file) = file {
          if let Ok(file_type) = file.file_type() {
            if file_type.is_file() {
              let mut temp_target_file = target_static_path.clone();
              temp_target_file.push(file.file_name().to_str().unwrap());
              std::fs::copy(file.path(), temp_target_file)?;
            }
          }
        }
      }
    }
  }

  println!("cargo:rerun-if-changed=build.rs");

  Ok(())
}
