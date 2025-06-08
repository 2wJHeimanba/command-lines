use command_lines::{Editor};
use command_lines::Result;

fn main() -> Result<()> {
    
  let mut editor = Editor::new()?;
  editor.run()

}
