use std::{fs::OpenOptions, io::{BufWriter, Write}, path::Path};

use crate::{error::{LibError, LibErrorContext}, model::{LibCell, LibGroup, LibLibrary, LibPin, LibValue}};

pub trait IntoLibGroup {
    fn into_group(self) -> LibGroup;
}

impl IntoLibGroup for LibLibrary {
    fn into_group(self) -> LibGroup {
        let mut groups = self.groups;
        for cell in self.cells {
            let cell_group = cell.into_group();
            groups.push(cell_group);
        }
        
        LibGroup { 
            kind: "library".to_string(), 
            params: vec![self.name],
            attrs: self.attrs,
            groups,
        }
    }  
}

impl IntoLibGroup for LibCell {
    fn into_group(self) -> LibGroup {
        let mut groups = self.groups;
        for pin in self.pins {
            let pin_group = pin.into_group();
            groups.push(pin_group);
        }
        
        LibGroup { 
            kind: "cell".to_string(), 
            params: vec![self.name], 
            attrs: self.attrs,
            groups,
        }
    }
}

impl IntoLibGroup for LibPin {
    fn into_group(self) -> LibGroup {
        let mut attrs = self.attrs;
        if let Some(function) = self.function {
            attrs.insert("function".to_string(), LibValue::String(function.to_string()));
        }

        LibGroup {
            kind: "pin".to_string(),
            params: vec![self.name],
            attrs,
            groups: self.groups
        }
    }
}

impl LibGroup {
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<(), LibError> {
        let file = OpenOptions::new()
            .create(true)
            .open(path).map_err(|e| LibError::Io(e))
            .context(|| "create file")?;
        let mut writer = BufWriter::new(file);

        self.write_to_file_ident(&mut writer, 0)
    }

    fn write_to_file_ident<W: Write>(&self, writer: &mut W, ident: usize) -> Result<(), LibError> {
        fn write_with_ident<W: Write>(writer: &mut W, ident: usize, add_ident: usize, context: &str) -> Result<(), LibError> {
            for _ in 0..ident + add_ident {
                writer.write_all(b"    ")?;
            }
            writer.write_all(context.as_bytes())?;
            writer.write_all(b"\n")?;
            Ok(())
        }

        // kind(params) {
        write_with_ident(writer, ident, 0, &format!("{}({}) {{", self.kind, self.params.join(", ")))?;
        
        // attr
        for (key, value) in self.attrs.iter() {
            if value.is_complex() {
                write_with_ident(writer, ident, 1, &format!("{} {};", key, value))?;
            } else {
                write_with_ident(writer, ident, 1, &format!("{}: {};", key, value))?;
            }
        }

        // groups, recursive call
        for group in self.groups.iter() {
            group.write_to_file_ident(writer, ident + 1)?;
        }

        // }
        write_with_ident(writer, ident, 0, "}")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_save_file() {

    }
}