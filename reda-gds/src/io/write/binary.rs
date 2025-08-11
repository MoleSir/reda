use std::{fs::File, io::Write, path::Path};
use crate::{GdsAref, GdsBoundary, GdsBox, GdsDateTime, GdsDbCoord, GdsFormat, GdsLibrary, GdsNode, GdsPath, GdsPathType, GdsPresentation, GdsSref, GdsStructure, GdsText, GdsTransform};
use crate::io::{record::GdsRecordType, GdsWriteResult};

pub struct GdsWriter<W> {
    writer: W,
}

impl GdsWriter<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> GdsWriteResult<Self> {
        let file = File::create(path)?;
        Ok(Self { writer: file })
    }
}

impl<W: Write> GdsWriter<W> {
    pub fn write(&mut self, gds: &GdsLibrary) -> GdsWriteResult<()> {
        self.write_header(&gds)?;
        self.write_library(gds)
    }
}

impl<W: std::io::Write> GdsWriter<W> {
    pub fn write_header(&mut self, gds: &GdsLibrary) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::Header, gds.version)
    }

    pub fn write_library(&mut self, gds: &GdsLibrary) -> GdsWriteResult<()> {
        self.write_begin_library(gds)?;
        self.write_library_name(gds)?;
        self.write_library_options(gds)?;
        self.write_units(gds)?;
        self.write_structures(gds)?;
        self.write_end_library()
    }

    pub fn write_begin_library(&mut self, lib: &GdsLibrary) -> GdsWriteResult<()> {
        self.write_datetime_record(GdsRecordType::BgnLib, &lib.create_date, &lib.modify_date)
    }

    pub fn write_library_name(&mut self, lib: &GdsLibrary) -> GdsWriteResult<()> {
        self.write_string_record(GdsRecordType::LibName, &lib.name)
    }

    pub fn write_library_options(&mut self, lib: &GdsLibrary) -> GdsWriteResult<()> {
        if let Some(ref reflibs) = lib.reflibs {
            self.write_reflibs(reflibs)?;
        }

        if let Some(ref fonts) = lib.fonts {
            self.write_fonts(fonts)?;
        }

        if let Some(ref attrtable) = lib.attrtable {
            self.write_attrtable(attrtable)?;
        }

        if let Some(generations) = lib.generations {
            self.write_generations(generations)?;
        }

        if let Some(fmt) = &lib.format {
            self.write_format(fmt)?;
        }

        Ok(())
    }

    pub fn write_reflibs(&mut self, reflibs: &[String; 2]) -> GdsWriteResult<()> {
        self.write_record(94, GdsRecordType::RefLibs)?;
        self.write_string_with_size(&reflibs[0], 45)?;
        self.write_string_with_size(&reflibs[1], 45)?;
        Ok(())
    }

    pub fn write_fonts(&mut self, fonts: &[String; 4]) -> GdsWriteResult<()> {
        self.write_record(4 * 44 + 4, GdsRecordType::Fonts)?;
        for font in fonts {
            self.write_string_with_size(font, 44)?;
        }
        Ok(())
    }

    pub fn write_format(&mut self, fmt: &GdsFormat) -> GdsWriteResult<()> {
        self.write_u16_record(GdsRecordType::Format, fmt.to_u16())
    }

    pub fn write_generations(&mut self, generations: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::Generations, generations)
    }

    pub fn write_attrtable(&mut self, attrtable: &str) -> GdsWriteResult<()> {
        self.write_record(48, GdsRecordType::AttrTable)?;
        self.write_string_with_size(attrtable, 44)?;
        Ok(())
    }

    pub fn write_units(&mut self, lib: &GdsLibrary) -> GdsWriteResult<()> {
        self.write_record(20, GdsRecordType::Uints)?;
        self.write_f64_ibm(lib.usrunits_per_dbunit)?;
        self.write_f64_ibm(lib.meters_per_dbunit)?;
        Ok(())
    }

    pub fn write_structures(&mut self, gds: &GdsLibrary) -> GdsWriteResult<()> {
        for structure in gds.structures.values() {
            self.write_structure(&structure.read().unwrap())?;
        }
        Ok(())
    }

    pub fn write_end_library(&mut self) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::EndLib)
    }
}


impl<W: std::io::Write> GdsWriter<W> {
    pub fn write_structure(&mut self, structure: &GdsStructure) -> GdsWriteResult<()> {
        self.write_structure_begin(structure)?;
        self.write_structure_name(structure)?;
        self.write_structure_elements(structure)?;
        self.write_structure_end()
    }

    pub fn write_structure_begin(&mut self, structure: &GdsStructure) -> GdsWriteResult<()> {
        self.write_datetime_record(GdsRecordType::BgnStr, &structure.create_date, &structure.modify_date)
    }

    pub fn write_structure_name(&mut self, structure: &GdsStructure) -> GdsWriteResult<()> {
        self.write_string_record(GdsRecordType::StrName, &structure.name)
    }

    pub fn write_structure_end(&mut self) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::EndStr)
    }

    pub fn write_structure_elements(&mut self, structure: &GdsStructure) -> GdsWriteResult<()> {
        for boundary in &structure.boundarys {
            self.write_boundary_element(boundary)?;
        }

        for path in &structure.paths {
            self.write_path_element(path)?;
        }

        for sref in &structure.srefs {
            self.write_sref_element(sref)?;
        }

        for aref in &structure.arefs {
            self.write_aref_element(aref)?;
        }

        for text in &structure.texts {
            self.write_text_element(text)?;
        }

        for node in &structure.nodes {
            self.write_node_element(node)?;
        }

        for bx in &structure.boxes {
            self.write_box_element(bx)?;
        }

        Ok(())
    }

    /// <boundary>: BOUNDARY [ELFLAGS] [PLEX] LAYER DATATYPE XY
    pub fn write_boundary_element(&mut self, boundary: &GdsBoundary) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::Boundary)?;
        if let Some(flags) = boundary.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = boundary.plex {
            self.write_plex_record(plex)?;
        }
        self.write_layer_record(boundary.layer)?;
        self.write_datatype_record(boundary.data_type)?;
        self.write_xy_record(&boundary.xy)?;
        self.write_element_end_record()
    }

    /// <path>: PATH [ELFLAGS] [PLEX] LAYER DATATYPE [PATHTYPE][WIDTH] XY
    pub fn write_path_element(&mut self, path: &GdsPath) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::Path)?;
        if let Some(flags) = path.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = path.plex {
            self.write_plex_record(plex)?;
        }
        self.write_layer_record(path.layer)?;
        self.write_datatype_record(path.data_type)?;
        self.write_pathtype_record(path.path_type)?;
        if let Some(width) = path.width {
            self.write_width_record(width)?;
        }
        self.write_xy_record(&path.xy)?;
        self.write_element_end_record()
    }

    /// <sref>:   SREF [ELFLAGS] [PLEX] SNAME [<strans>] XY
    /// <strans>: STRANS [MAG] [ANGLE]
    pub fn write_sref_element(&mut self, sref: &GdsSref) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::SRef)?;
        if let Some(flags) = sref.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = sref.plex {
            self.write_plex_record(plex)?;
        }
        self.write_sname_record(&sref.s_name)?;
        if let Some(transform) = &sref.transform {
            self.write_transform_record(transform)?;
        }
        self.write_xy_record(&sref.xy)?;
        self.write_element_end_record()
    }

    /// <aref>:   AREF [ELFLAGS] [PLEX] SNAME [<strans>] COLROW XY
    /// <strans>: STRANS [MAG] [ANGLE]
    pub fn write_aref_element(&mut self, aref: &GdsAref) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::ARef)?;
        if let Some(flags) = aref.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = aref.plex {
            self.write_plex_record(plex)?;
        }
        if let Some(transform) = &aref.transform {
            self.write_transform_record(transform)?;
        }
        self.write_colrow_record(aref.col, aref.row)?;
        self.write_xy_record(&aref.xy)?;
        self.write_element_end_record()
    }

    /// <text>:     TEXT [ELFLAGS] [PLEX] LAYER <textbody>
    /// <textbody>: TEXTYPE [PRESENTATION] [PATHTYPE] [WIDTH] [<strans>] XY STRING
    /// <strans>:   STRANS [MAG] [ANGLE]
    pub fn write_text_element(&mut self, text: &GdsText) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::Text)?;
        if let Some(flags) = text.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = text.plex {
            self.write_plex_record(plex)?;
        }
        self.write_layer_record(text.layer)?;
        self.write_texttype_record(text.text_type)?;
        if let Some(pres) = &text.presentation {
            self.write_presentation_record(pres)?;
        }
        self.write_pathtype_record(text.path_type)?;
        if let Some(width) = text.width {
            self.write_width_record(width)?;
        }
        if let Some(transform) = &text.transform {
            self.write_transform_record(transform)?;
        }
        self.write_xy_record(&text.xy)?;
        self.write_ascii_string_record(&text.string)?;
        self.write_element_end_record()
    }

    /// <node>: NODE [ELFLAGS]. [PLEX] LAYER NODETYPE XY
    pub fn write_node_element(&mut self, node: &GdsNode) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::Node)?;
        if let Some(flags) = node.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = node.plex {
            self.write_plex_record(plex)?;
        }
        self.write_layer_record(node.layer)?;
        self.write_nodetype_record(node.node_type)?;
        self.write_xy_record(&node.xy)?;
        self.write_element_end_record()
    }

    /// <box>: NODE [ELFLAGS]. [PLEX] LAYER BOXTYPE XY
    pub fn write_box_element(&mut self, bx: &GdsBox) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::Box)?;
        if let Some(flags) = bx.elf_flags {
            self.write_elflags_record(flags)?;
        }
        if let Some(plex) = bx.plex {
            self.write_plex_record(plex)?;
        }
        self.write_layer_record(bx.layer)?;
        self.write_boxtype_record(bx.box_type)?;
        self.write_xy_record(&bx.xy)?;
        self.write_element_end_record()
    }
}

/// Method to write specify record
impl<W: Write> GdsWriter<W> {
    pub fn write_element_end_record(&mut self) -> GdsWriteResult<()> {
        self.write_empty_record(GdsRecordType::EndEle)
    }

    pub fn write_elflags_record(&mut self, elf_flags: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::ElFlags, elf_flags)
    }

    pub fn write_plex_record(&mut self, plex: i32) -> GdsWriteResult<()> {
        self.write_i32_record(GdsRecordType::Plex, plex)
    }

    pub fn write_layer_record(&mut self, layer: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::Layer, layer)
    }

    pub fn write_datatype_record(&mut self, data_type: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::DataType, data_type)
    }

    pub fn write_xy_record(&mut self, coords: &[GdsDbCoord]) -> GdsWriteResult<()> {
        let record_size = 4 + coords.len() * 8;
        self.write_record(record_size, GdsRecordType::Xy)?;
        for coord in coords {
            self.write_i32(coord.x)?;
            self.write_i32(coord.y)?;
        }
        Ok(())
    }

    pub fn write_pathtype_record(&mut self, path_type: GdsPathType) -> GdsWriteResult<()> {
        self.write_u16_record(GdsRecordType::PathType, path_type.to_u16())
    }

    pub fn write_width_record(&mut self, width: i32) -> GdsWriteResult<()> {
        self.write_i32_record(GdsRecordType::Width, width)
    }

    pub fn write_sname_record(&mut self, sname: &str) -> GdsWriteResult<()> {
        self.write_string_record(GdsRecordType::SName, sname)
    }

    pub fn write_transform_record(&mut self, tranform: &GdsTransform) -> GdsWriteResult<()> {
        let value = tranform.flag.to_u16();
        self.write_u16_record(GdsRecordType::STrans, value)?;

        if tranform.magnification != 1.0 {
            self.write_f64_record(GdsRecordType::Mag, tranform.magnification)?;
        }

        if tranform.angle != 0.0 {
            self.write_f64_record(GdsRecordType::Angle, tranform.angle)?;
        }

        Ok(())
    }

    pub fn write_colrow_record(&mut self, col: i16, row: i16) -> GdsWriteResult<()> {
        self.write_record(8, GdsRecordType::ColRow)?;
        self.write_i16(col)?;
        self.write_i16(row)?;
        Ok(())
    }

    pub fn write_texttype_record(&mut self, text_type: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::TextType, text_type)
    }

    pub fn write_presentation_record(&mut self, presentation: &GdsPresentation) -> GdsWriteResult<()> {
        self.write_u16_record(GdsRecordType::Presentation, presentation.to_u16())
    }

    pub fn write_ascii_string_record(&mut self, string: &str) -> GdsWriteResult<()> {
        self.write_string_record(GdsRecordType::String, string)
    }

    pub fn write_nodetype_record(&mut self, node_type: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::NodeType, node_type)
    }

    pub fn write_boxtype_record(&mut self, box_type: i16) -> GdsWriteResult<()> {
        self.write_i16_record(GdsRecordType::BoxType, box_type)
    }
}

/// Method to write record
impl<W: Write> GdsWriter<W> {
    pub fn write_i16_record(&mut self, record_type: GdsRecordType, value: i16) -> GdsWriteResult<()> {
        self.write_record(6, record_type)?;
        self.write_i16(value)?;
        Ok(())
    }

    pub fn write_u16_record(&mut self, record_type: GdsRecordType, value: u16) -> GdsWriteResult<()> {
        self.write_record(6, record_type)?;
        self.write_u16(value)?;
        Ok(())
    }

    pub fn write_i32_record(&mut self, record_type: GdsRecordType, value: i32) -> GdsWriteResult<()> {
        self.write_record(8, record_type)?;
        self.write_i32(value)?;
        Ok(())
    }

    pub fn write_f64_record(&mut self, record_type: GdsRecordType, value: f64) -> GdsWriteResult<()> {
        self.write_record(12, record_type)?;
        self.write_f64_ibm(value)?;
        Ok(())
    }

    pub fn write_string_record(&mut self, record_type: GdsRecordType, value: &str) -> GdsWriteResult<()> {
        let mut size = value.len();
        if size % 2 != 0 {
            size += 1;
        }
        let total_size = size + 4;
        self.write_record(total_size, record_type)?;
        self.write_string(value)?;
        Ok(())
    }

    pub fn write_empty_record(&mut self, record_type: GdsRecordType) -> GdsWriteResult<()> {
        self.write_record(4, record_type)
    }

    pub fn write_datetime_record(
        &mut self,
        record_type: GdsRecordType,
        created: &GdsDateTime,
        modified: &GdsDateTime,
    ) -> GdsWriteResult<()> {
        self.write_record(28, record_type)?;
        self.write_datetime(created)?;
        self.write_datetime(modified)?;
        Ok(())
    }

    fn write_record(&mut self, size: usize, tp: GdsRecordType) -> GdsWriteResult<()> {
        self.write_u16(size as u16)?;
        self.write_u16(tp as u16)?;
        Ok(())
    }
}

/// Method to write data
impl<W: Write> GdsWriter<W> {
    fn write_u16(&mut self, value: u16) -> GdsWriteResult<()> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }
    
    fn write_i16(&mut self, value: i16) -> GdsWriteResult<()> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn write_i32(&mut self, value: i32) -> GdsWriteResult<()> {
        self.writer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    fn write_string(&mut self, string: &str) -> GdsWriteResult<()> {
        self.writer.write_all(string.as_bytes())?;
        if string.len() % 2 != 0 {
            self.writer.write_all(&[0])?;
        }
        Ok(())
    }

    fn write_string_with_size(&mut self, string: &str, size: usize) -> GdsWriteResult<()> {
        let bytes = string.as_bytes();
        let len = bytes.len();
        if len < size {
            self.writer.write_all(bytes)?;
            self.writer.write_all(&vec![0; size - len])?;
        } else {
            self.writer.write_all(&bytes[..size])?;
        }
        Ok(())
    }

    fn write_f64_ibm(&mut self, value: f64) -> GdsWriteResult<()> {
        let mut sign = 0i64;
        let mut exponent = 0i64;
        let mut mantissa = 0i64;

        if value != 0.0 {
            let bits = value.to_bits() as i64;
            sign = (bits >> 63) & 0x1;
            exponent = ((bits >> 52) & 0x7ff) - 1023;
            mantissa = (bits & 0x000f_ffff_ffff_ffff) | 0x0010_0000_0000_0000;
            mantissa <<= 11; // left align
            mantissa >>= 1;  // discard one bit

            let shift = (-exponent) & 0x3;
            for _ in 0..shift {
                mantissa >>= 1;
            }

            exponent = (exponent + 3) >> 2;
            exponent += 64;
        }

        let ibm = ((sign << 63) | (exponent << 56) | ((mantissa >> 8) & 0x00ff_ffff_ffff_ffff)) as u64;
        let ibm_bytes = ibm.to_be_bytes();
        self.writer.write_all(&ibm_bytes)?;

        Ok(())
    }

    fn write_datetime(&mut self, datetime: &GdsDateTime) -> GdsWriteResult<()> {
        self.write_i16(datetime.year)?;
        self.write_i16(datetime.month)?;
        self.write_i16(datetime.day)?;
        self.write_i16(datetime.hour)?;
        self.write_i16(datetime.minute)?;
        self.write_i16(datetime.second)?; 
        Ok(())
    }
}