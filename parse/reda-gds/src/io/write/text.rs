use std::{fs::File, path::Path};
use crate::{GdsAref, GdsBoundary, GdsBox, GdsLibrary, GdsNode, GdsPath, GdsSref, GdsStructure, GdsText};

use super::GdsWriteResult;

pub struct TextWriter<W> {
    writer: W,
}

impl TextWriter<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> GdsWriteResult<Self> {
        let file = File::create(path)?;
        Ok(Self { writer: file })
    }
}

impl<W: std::io::Write> TextWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: std::io::Write> TextWriter<W> {
    pub fn write(&mut self, layout: &GdsLibrary) -> GdsWriteResult<()> {
        let indent = 0;
        self.write_indent(indent)?;
        writeln!(self.writer, "GDSII Layout Object")?;

        let attr_indent = indent + 1;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "version: {}", layout.version)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "library name: {}", layout.name)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "units: ({}, {})", layout.usrunits_per_dbunit, layout.meters_per_dbunit)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "create date: {}", layout.create_date.to_string())?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "modify date: {}", layout.modify_date.to_string())?;

        for (_key, structure) in &layout.structures {
            self.write_structure(&structure.read().unwrap(), attr_indent)?;
        }

        Ok(())
    }
}


impl<W: std::io::Write> TextWriter<W> {
    pub fn write_structure(&mut self, structure: &GdsStructure, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Structure:")?;

        let attr_indent = indent + 1;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "name: {}", structure.name)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "create date: {}", structure.create_date.to_string())?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "modify date: {}", structure.modify_date.to_string())?;

        for e in &structure.boundarys {
            self.write_boundary(e, attr_indent)?;
        }

        for e in &structure.paths {
            self.write_path(e, attr_indent)?;
        }

        for e in &structure.srefs {
            self.write_sref(e, attr_indent)?;
        }

        for e in &structure.arefs {
            self.write_aref(e, attr_indent)?;
        }

        for e in &structure.texts {
            self.write_text(e, attr_indent)?;
        }

        for e in &structure.nodes {
            self.write_node(e, attr_indent)?;
        }

        for e in &structure.boxes {
            self.write_box(e, attr_indent)?;
        }

        Ok(())
    }
}


impl<W: std::io::Write> TextWriter<W> {
    pub fn write_boundary(&mut self, boundary: &GdsBoundary, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Boundary Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = boundary.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = boundary.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "layer: {}", boundary.layer)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "data_type: {}", boundary.data_type)?;

        self.write_indent(attr_indent)?;
        write!(self.writer, "xy: [")?;
        for (i, coord) in boundary.xy.iter().enumerate() {
            if i > 0 {
                write!(self.writer, ", ")?;
            }
            write!(self.writer, "({}, {})", coord.x, coord.y)?;
        }
        writeln!(self.writer, "]")?;

        Ok(())
    }

    pub fn write_path(&mut self, path: &GdsPath, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Path Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = path.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = path.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "layer: {}", path.layer)?;

        if let Some(purpose) = path.purpose_layer {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "purpose layer: {}", purpose)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "data_type: {}", path.data_type)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "path_type: {}", path.path_type)?;

        if let Some(width) = path.width {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "width: {}", width)?;
        }

        self.write_indent(attr_indent)?;
        write!(self.writer, "xy: [")?;
        for (i, coord) in path.xy.iter().enumerate() {
            if i > 0 {
                write!(self.writer, ", ")?;
            }
            write!(self.writer, "({}, {})", coord.x, coord.y)?;
        }
        writeln!(self.writer, "]")?;

        Ok(())
    }

    pub fn write_sref(&mut self, sref: &GdsSref, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Sref Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = sref.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = sref.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "s_name: {}", sref.s_name)?;

        if let Some(transform) = &sref.transform {
            self.write_indent(attr_indent)?;
            write!(self.writer, "s_trans:")?;
            if transform.flag.reflect {
                write!(self.writer, " x_mirror")?;
            }
            if transform.flag.absolute_magnification {
                write!(self.writer, "absolute magnification")?;
            }
            if transform.flag.absolute_angle {
                write!(self.writer, "absolute angle")?;
            }
            writeln!(self.writer)?;
        
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "mag: {}", transform.magnification())?;
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "angle: {}", transform.angle())?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "coordinate: [{}, {}]", sref.position.x, sref.position.y)?;

        Ok(())
    }

    pub fn write_aref(&mut self, aref: &GdsAref, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Aref Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = aref.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = aref.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "s_name: {}", aref.s_name)?;

        if let Some(transform) = &aref.transform {
            self.write_indent(attr_indent)?;
            write!(self.writer, "s_trans:")?;
            if transform.flag.reflect {
                write!(self.writer, " x_mirror")?;
            }
            if transform.flag.absolute_magnification {
                write!(self.writer, "absolute magnification")?;
            }
            if transform.flag.absolute_angle {
                write!(self.writer, "absolute angle")?;
            }
            writeln!(self.writer)?;
        
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "mag: {}", transform.magnification())?;
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "angle: {}", transform.angle())?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "col: {}", aref.col)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "row: {}", aref.row)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "coordinate: [{}, {}]", aref.position.x, aref.position.y)?;

        Ok(())
    }

    pub fn write_text(&mut self, text: &GdsText, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Text Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = text.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = text.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "layer: {}", text.layer)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "data_type: {}", text.text_type)?;

        if let Some(presentation) = text.presentation {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "presentation: {}", presentation)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "path_type: {}", text.path_type)?;

        if let Some(width) = text.width {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "width: {}", width)?;
        }

        if let Some(transform) = &text.transform {
            self.write_indent(attr_indent)?;
            write!(self.writer, "s_trans:")?;
            if transform.flag.reflect {
                write!(self.writer, " x_mirror")?;
            }
            if transform.flag.absolute_magnification {
                write!(self.writer, "absolute magnification")?;
            }
            if transform.flag.absolute_angle {
                write!(self.writer, "absolute angle")?;
            }
            writeln!(self.writer)?;
            
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "mag: {}", transform.magnification())?;
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "angle: {}", transform.angle())?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "coordinate: [{}, {}]", text.position.x, text.position.y)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "string: {}", text.string)?;

        Ok(())
    }

    pub fn write_node(&mut self, node: &GdsNode, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Node Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = node.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = node.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "layer: {}", node.layer)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "data_type: {}", node.node_type)?;

        self.write_indent(attr_indent)?;
        write!(self.writer, "xy: [")?;
        for coord in &node.xy {
            write!(self.writer, "({}, {}), ", coord.x, coord.y)?;
        }
        writeln!(self.writer, "]")?;

        Ok(())
    }

    pub fn write_box(&mut self, boxx: &GdsBox, indent: usize) -> GdsWriteResult<()> {
        self.write_indent(indent)?;
        writeln!(self.writer, "Box Element")?;

        let attr_indent = indent + 1;

        if let Some(flags) = boxx.elf_flags {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "elf_flags: {}", flags)?;
        }

        if let Some(plex) = boxx.plex {
            self.write_indent(attr_indent)?;
            writeln!(self.writer, "plex: {}", plex)?;
        }

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "layer: {}", boxx.layer)?;

        self.write_indent(attr_indent)?;
        writeln!(self.writer, "box_type: {}", boxx.box_type)?;

        self.write_indent(attr_indent)?;
        write!(self.writer, "xy: [")?;
        for coord in &boxx.xy {
            write!(self.writer, "({}, {}), ", coord.x, coord.y)?;
        }
        writeln!(self.writer, "]")?;

        Ok(())
    }
}

impl<W: std::io::Write> TextWriter<W> {
    fn write_indent(&mut self, level: usize) -> GdsWriteResult<()> {
        for _ in 0..level {
            write!(self.writer, "    ")?;
        }
        Ok(())
    }
}