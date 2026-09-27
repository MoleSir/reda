use std::path::Path;

use write::IntoLibGroup;

use crate::{error::{LibError, LibErrorContext}, model::LibLibrary};

pub mod read;
pub mod write;


impl LibLibrary {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, LibError> {
        let path = path.as_ref();
        let context = std::fs::read_to_string(path)
            .map_err(LibError::Io)
            .context(|| format!("read file{:?}", path))?;
        let ast = read::parse_liberty(&context)
            .context(|| "parse source to ast")?;
        Self::from_ast(ast)
            .context(|| "semantic ast to model")
    }

    // TODO: just borrow self!
    pub fn save_file<P: AsRef<Path>>(self, path: P) -> Result<(), LibError> {
        let group = self.into_group();
        group.save_file(path)
    }
}