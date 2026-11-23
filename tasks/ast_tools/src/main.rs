use std::error::Error;

use robusta_ast_tools::{workspace_root, write_generated};

fn main() -> Result<(), Box<dyn Error>> {
    write_generated(&workspace_root())?;
    Ok(())
}
