mod error;

pub use error::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, RwLock};

use crate::{
    GdsAref, GdsArefBuilder, GdsArefBuilderError, GdsBoundary, GdsBoundaryBuilder, GdsBoundaryBuilderError, GdsBox, GdsBoxBuilder, GdsBoxBuilderError, GdsDateTime, GdsDbCoord, GdsFormat, GdsLibrary, GdsLibraryBuilder, GdsNode, GdsNodeBuilder, GdsNodeBuilderError, GdsPath, GdsPathBuilder, GdsPathBuilderError, GdsPathType, GdsPresentation, GdsSref, GdsSrefBuilder, GdsSrefBuilderError, GdsStructure, GdsText, GdsTextBuilder, GdsTextBuilderError, GdsTransform
};
use super::record::GdsRecordType;

pub struct GdsReader<R> {
    reader: BufReader<R>,
}

impl GdsReader<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> GdsReadResult<Self> {
        let reader = BufReader::new(File::open(path)?);
        Ok(Self { reader })
    }
}

// impl<R: Read + Seek> GdsReader<R> {
//     pub fn with_reader(reader: R) -> GdsReadResult<Self> {
//         Ok(Self { reader: BufReader::new(reader) })
//     }
// }

impl<R: Read + Seek> GdsReader<R> {
    pub fn read(&mut self) -> GdsReadResult<GdsLibrary> {
        match self.read_impl() {
            Ok(gds) => Ok(gds),
            Err(e) => {
                let context = match self.reader.stream_position() {
                    Ok(pos) => format!("read until {pos} bytes"),
                    Err(e) => format!("error to get bytes bias for '{e}'"),
                };
                Err(e.wrap(context)) 
            }
        }
    }

    fn read_impl(&mut self) -> GdsReadResult<GdsLibrary> {
        let mut builder = GdsLibraryBuilder::default();
        self.read_header(&mut builder).map_err(|e| e.wrap("read header"))?;
        self.read_library(&mut builder).map_err(|e| e.wrap("read Library"))?;
        Ok(builder.build().unwrap())
    }
}

impl<R: Read + Seek> GdsReader<R> {
    fn read_header(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record(6, GdsRecordType::Header)?;
        builder.version(self.take_i16_record()?);
        Ok(())
    }
}

impl<R: Read + Seek> GdsReader<R> {
    fn read_library(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.read_library_begin(builder).map_err(|e| e.wrap("read library begin"))?;

        self.read_library_name(builder).map_err(|e| e.wrap("read library name"))?;
        self.read_library_options(builder).map_err(|e| e.wrap(""))?;
        self.read_units(builder).map_err(|e| e.wrap("read units"))?;

        let structures = self.read_structures().map_err(|e| e.wrap("read structures"))?;
        builder.structures(structures);
        
        self.read_library_end(builder).map_err(|e| e.wrap("read library end"))?;
        Ok(())
    }

    fn read_library_begin(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record(28, GdsRecordType::BgnLib)?;
        self.jump_bytes(4)?;

        builder.create_date(self.take_datetime()?);
        builder.modify_date(self.take_datetime()?);

        Ok(())   
    }

    fn read_library_name(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record_type(GdsRecordType::LibName)?;
        builder.name(self.take_string_record()?);
        Ok(())
    }

    fn read_units(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record(20, GdsRecordType::Uints)?;
        self.jump_bytes(4)?;

        builder.usrunits_per_dbunit(self.take_f64()?);
        builder.meters_per_dbunit(self.take_f64()?);

        Ok(())
    }

    fn read_library_end(&mut self, _builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record(4, GdsRecordType::EndLib)?;
        self.jump_bytes(4)?;
        Ok(())
    }
}

impl<R: Read + Seek> GdsReader<R> {
    fn read_library_options(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        loop {
            let tp = self.peek_record_type()?;
            match tp {
                GdsRecordType::RefLibs => self.read_reflibs(builder).map_err(|e| e.wrap("read reflibs"))?,
                GdsRecordType::Fonts => self.read_fonts(builder).map_err(|e| e.wrap("read reflibs"))?,
                GdsRecordType::AttrTable => self.read_attrtable(builder).map_err(|e| e.wrap("read attrtable"))?,
                GdsRecordType::Generations => self.read_generations(builder).map_err(|e| e.wrap("read generations"))?,
                GdsRecordType::Format => self.read_format(builder).map_err(|e| e.wrap("read format"))?,
                _ => return Ok(())
            }
        }
    }

    pub fn read_reflibs(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record(94, GdsRecordType::RefLibs)?;
        self.jump_bytes(4)?;
        builder.reflibs([
            self.take_string(45)?,
            self.take_string(45)?,
        ]);
        Ok(())
    }

    pub fn read_fonts(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record_size(4 * 44 + 4)?;
        self.jump_bytes(4)?;
        builder.fonts([
            self.take_string(44)?,
            self.take_string(44)?,
            self.take_string(44)?,
            self.take_string(44)?,
        ]);
        Ok(())
    }

    pub fn read_attrtable(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        self.ensure_record_size(48)?;
        self.jump_bytes(4)?;
        builder.attrtable(self.take_string(44)?);
        Ok(())
    }

    pub fn read_generations(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        builder.generations(self.take_i16_record()?);
        Ok(())
    }

    pub fn read_format(&mut self, builder: &mut GdsLibraryBuilder) -> GdsReadResult<()> {
        let value = self.take_u16_record()?;
        let fmt = GdsFormat::from_u16(value)
            .ok_or_else(|| GdsReadError::InvalidFormat(value))?;
        builder.format(fmt);
        Ok(())
    }
}

macro_rules! read_optional_field {
    ($builder:ident.$field:ident <- $this:ident.$method:ident if $record_type:ident) => {
        if $this.peek_record_type()? == GdsRecordType::$record_type {
            $builder.$field($this.$method()?);
        }
    };
}


macro_rules! read_required_field {
    (
        $builder:ident.$field:ident <- $this:ident.$method:ident if $variant:ident =>
        $read_error_variant:ident ( $builder_error_ty:ty )
    ) => {
        if $this.peek_record_type()? == GdsRecordType::$variant {
            $builder.$field($this.$method()?);
        } else {
            return Err(GdsReadError::$read_error_variant(
                <$builder_error_ty>::UninitializedField(stringify!($field))
            ));
        }
    };
}

impl<R: Read + Seek> GdsReader<R> {
    fn read_structures(&mut self) -> GdsReadResult<HashMap<String, Arc<RwLock<GdsStructure>>>> {
        let mut structures = HashMap::new();
        let mut size = 0;
        while self.check_record_type(GdsRecordType::BgnStr)? {
            let structure = self.read_structure().map_err(|e| e.wrap(format!("read {size} structure")))?;
            let name = structure.name.clone();
            structures.insert(name, Arc::new(RwLock::new(structure)));
            size += 1;
        }
        Ok(structures)
    }

    fn read_structure(&mut self) -> GdsReadResult<GdsStructure> {
        let mut s = GdsStructure::default();

        self.read_structure_begin(&mut s).map_err(|e| e.wrap("read structure begin"))?;
        self.read_structure_name(&mut s).map_err(|e| e.wrap("read structure name"))?;

        self.read_structure_elements(&mut s).map_err(|e| e.wrap("read structure elements"))?;
        self.read_structure_end().map_err(|e| e.wrap("read structure end"))?;

        Ok(s)
    }

    fn read_structure_begin(&mut self, structure: &mut GdsStructure) -> GdsReadResult<()> {
        self.ensure_record(28, GdsRecordType::BgnStr)?;
        self.jump_bytes(4)?;
        structure.modify_date = self.take_datetime()?;
        structure.create_date = self.take_datetime()?;
        Ok(())
    }

    fn read_structure_name(&mut self, structure: &mut GdsStructure) -> GdsReadResult<()> {
        self.ensure_record_type(GdsRecordType::StrName)?;
        structure.name = self.take_string_record()?;
        Ok(())
    }

    fn read_structure_end(&mut self) -> GdsReadResult<()> {
        self.ensure_record(4, GdsRecordType::EndStr)?;
        self.jump_bytes(4)?;
        Ok(())
    }

    fn read_structure_elements(&mut self, s: &mut GdsStructure) -> GdsReadResult<()> {
        loop {
            match self.peek_record_type()? {
                GdsRecordType::Boundary => 
                    s.boundarys.push(self.read_element_boundary().map_err(|e| e.wrap("read boundary"))?),
                GdsRecordType::Path => 
                    s.paths.push(self.read_element_path().map_err(|e| e.wrap("read path"))?),
                GdsRecordType::Text => 
                    s.texts.push(self.read_element_text().map_err(|e| e.wrap("read text"))?),
                GdsRecordType::ARef => 
                    s.arefs.push(self.read_element_aref().map_err(|e| e.wrap("read aref"))?),
                GdsRecordType::SRef => 
                    s.srefs.push(self.read_element_sref().map_err(|e| e.wrap("read sref"))?),
                GdsRecordType::Node => 
                    s.nodes.push(self.read_element_node().map_err(|e| e.wrap("read node"))?),
                GdsRecordType::Box => 
                    s.boxes.push(self.read_element_box().map_err(|e| e.wrap("read box"))?),
                _ => break
            }
        }
        Ok(())
    }

    fn read_element_boundary(&mut self) -> GdsReadResult<GdsBoundary> {
        self.read_element_header()?;
        let mut builder = GdsBoundaryBuilder::default();

        read_optional_field!(builder.elf_flags <- self.take_i16_record if ElFlags);
        read_optional_field!(builder.plex      <- self.take_i32_record if Plex);
        read_required_field!(builder.layer     <- self.take_i16_record     if Layer     => BuildBoundary(GdsBoundaryBuilderError));
        read_required_field!(builder.data_type <- self.take_i16_record if DataType  => BuildBoundary(GdsBoundaryBuilderError));
        read_required_field!(builder.xy        <- self.read_xy        if Xy        => BuildBoundary(GdsBoundaryBuilderError));
    
        self.read_element_end()?;
        Ok(builder.build()?)
    }

    pub fn read_element_path(&mut self) -> GdsReadResult<GdsPath> {
        self.read_element_header()?;
        let mut builder = GdsPathBuilder::default();

        read_optional_field!(builder.elf_flags      <- self.take_i16_record    if ElFlags);
        read_optional_field!(builder.plex           <- self.take_i32_record    if Plex);
        read_required_field!(builder.layer          <- self.take_i16_record    if Layer     => BuildPath(GdsPathBuilderError));
        read_required_field!(builder.data_type      <- self.take_i16_record    if DataType  => BuildPath(GdsPathBuilderError));
        read_optional_field!(builder.path_type      <- self.read_path_type     if PathType);
        read_optional_field!(builder.width          <- self.take_i32_record    if Width);
        read_required_field!(builder.xy             <- self.read_xy            if Xy        => BuildPath(GdsPathBuilderError));
        read_optional_field!(builder.purpose_layer  <- self.take_i16_record    if TextType);

        if self.peek_record_type()? == GdsRecordType::BgnExtn {
            self.take_i32_record()?;
        }
        if self.peek_record_type()? == GdsRecordType::EndExtn {
            self.take_i32_record()?;
        }

        self.read_element_end()?;
        Ok(builder.build()?)
    }

    pub fn read_element_sref(&mut self) -> GdsReadResult<GdsSref> {
        self.read_element_header()?;
        let mut builder = GdsSrefBuilder::default();

        read_optional_field!(builder.elf_flags <- self.take_i16_record      if ElFlags);
        read_optional_field!(builder.plex      <- self.take_i32_record      if Plex);
        read_required_field!(builder.s_name    <- self.take_string_record   if SName     => BuildSref(GdsSrefBuilderError));
        read_optional_field!(builder.transform <- self.read_transform       if STrans);
        read_required_field!(builder.xy        <- self.read_xy              if Xy        => BuildSref(GdsSrefBuilderError));

        self.read_element_end()?;
        Ok(builder.build()?)
    }

    pub fn read_element_aref(&mut self) -> GdsReadResult<GdsAref> {
        self.read_element_header()?;
        let mut builder = GdsArefBuilder::default();

        read_optional_field!(builder.elf_flags <- self.take_i16_record      if ElFlags);
        read_optional_field!(builder.plex      <- self.take_i32_record      if Plex);
        read_required_field!(builder.s_name    <- self.take_string_record   if SName     => BuildAref(GdsArefBuilderError));
        read_optional_field!(builder.transform <- self.read_transform       if STrans);

        if self.peek_record_type()? == GdsRecordType::ColRow {
            let (col, row) = self.read_col_row()?;
            builder.col(col);
            builder.row(row);
        }
        read_required_field!(builder.xy        <- self.read_xy        if Xy        => BuildAref(GdsArefBuilderError));

        self.read_element_end()?;
        Ok(builder.build()?)
    }

    pub fn read_element_text(&mut self) -> GdsReadResult<GdsText> {
        self.read_element_header()?;
        let mut builder = GdsTextBuilder::default();

        read_optional_field!(builder.plex          <- self.take_i32_record     if Plex);
        read_optional_field!(builder.elf_flags     <- self.take_i16_record     if ElFlags);
        read_required_field!(builder.layer         <- self.take_i16_record     if Layer     => BuildText(GdsTextBuilderError));
        read_required_field!(builder.text_type     <- self.take_i16_record     if TextType  => BuildText(GdsTextBuilderError));
        read_optional_field!(builder.presentation  <- self.read_presentation   if Presentation);
        read_optional_field!(builder.path_type     <- self.read_path_type      if PathType);
        read_optional_field!(builder.width         <- self.take_i32_record     if Width);
        read_optional_field!(builder.transform     <- self.read_transform      if STrans);
        read_required_field!(builder.xy            <- self.read_xy             if Xy        => BuildText(GdsTextBuilderError));
        read_required_field!(builder.string        <- self.take_string_record  if String    => BuildText(GdsTextBuilderError));

        self.read_element_end()?;
        Ok(builder.build()?)
    }

    pub fn read_element_node(&mut self) -> GdsReadResult<GdsNode> {
        self.read_element_header()?;
        let mut builder = GdsNodeBuilder::default();

        read_optional_field!(builder.elf_flags     <- self.take_i16_record     if ElFlags);
        read_optional_field!(builder.plex          <- self.take_i32_record     if Plex);
        read_required_field!(builder.layer         <- self.take_i16_record     if Layer     => BuildNode(GdsNodeBuilderError));
        read_required_field!(builder.node_type     <- self.take_i16_record     if Layer     => BuildNode(GdsNodeBuilderError));
        read_required_field!(builder.xy            <- self.read_xy             if Layer     => BuildNode(GdsNodeBuilderError));

        self.read_element_end()?;
        Ok(builder.build()?)
    }

    pub fn read_element_box(&mut self) -> GdsReadResult<GdsBox> {
        self.read_element_header()?;
        let mut builder = GdsBoxBuilder::default();

        read_optional_field!(builder.elf_flags     <- self.take_i16_record     if ElFlags);
        read_optional_field!(builder.plex          <- self.take_i32_record     if Plex);
        read_required_field!(builder.layer         <- self.take_i16_record     if Layer     => BuildBox(GdsBoxBuilderError));
        read_required_field!(builder.box_type      <- self.take_i16_record     if Layer     => BuildBox(GdsBoxBuilderError));
        read_required_field!(builder.xy            <- self.read_xy             if Layer     => BuildBox(GdsBoxBuilderError));

        self.read_element_end()?;
        Ok(builder.build()?)
    }

    fn read_element_header(&mut self) -> GdsReadResult<()> {
        self.ensure_record_size(4)?;
        self.jump_bytes(4)?;
        Ok(())
    }

    fn read_element_end(&mut self) -> GdsReadResult<()> {
        self.ensure_record(4, GdsRecordType::EndEle)?;
        self.jump_bytes(4)?;
        Ok(())
    }

    pub fn read_col_row(&mut self) -> GdsReadResult<(i16, i16)> {
        self.ensure_record_size(8)?;
        self.jump_bytes(4)?;
        let col = self.take_i16()?;
        let row = self.take_i16()?;
        Ok((col, row))
    }

    pub fn read_xy(&mut self) -> GdsReadResult<Vec<GdsDbCoord>> {
        let mut header = [0u8; 4];
        self.reader.read_exact(&mut header)?;
        let record_size = u16::from_be_bytes([header[0], header[1]]) as usize;
        let record_type = u16::from_be_bytes([header[2], header[3]]);
        let rec_type = GdsRecordType::from_u16(record_type)
            .ok_or(GdsReadError::UnsupportRecordType(record_type))?;

        if rec_type != GdsRecordType::Xy {
            return Err(GdsReadError::UnexpectRecordType(GdsRecordType::Xy, rec_type));
        }

        if record_size < 4 || (record_size - 4) % 8 != 0 {
            return Err(GdsReadError::InvalidRecordSize(record_size));
        }

        let position_count = (record_size - 4) / 8;
        let mut coords = Vec::with_capacity(position_count);
        for _ in 0..position_count {
            let x = self.take_i32()?;
            let y = self.take_i32()?;
            coords.push(GdsDbCoord { x, y });
        }
        Ok(coords)
    }

    pub fn read_transform(&mut self) -> GdsReadResult<GdsTransform> {
        // Take the u16 value as flag
        let value = self.take_u16_record()? as u16;
        let mut transform = GdsTransform::with_flag(value);

        // Is mag?
        if self.peek_record_type()? == GdsRecordType::Mag {
            transform.magnification = self.take_f64_record()?;
        }

        // If angle
        if self.peek_record_type()? == GdsRecordType::Angle {
            transform.angle = self.take_f64_record()?;
        }

        Ok(transform)
    }   

    pub fn read_path_type(&mut self) -> GdsReadResult<GdsPathType> {
        let value = self.take_u16_record()?;
        GdsPathType::from_u16(value)
            .ok_or_else(|| GdsReadError::BuildText(GdsTextBuilderError::ValidationError(format!("'{value}' is not an valid path type"))))
    }

    pub fn read_presentation(&mut self) -> GdsReadResult<GdsPresentation> {
        let value = self.take_u16_record()?;
        GdsPresentation::from_u16(value)
            .map_err(|e| GdsReadError::BuildText(GdsTextBuilderError::ValidationError(e)))
    }
}

/// These method do not check record type, please ensure it correct!
/// And will take all record!
impl<R: Read + Seek> GdsReader<R> {
    fn take_u16_record(&mut self) -> GdsReadResult<u16> {
        self.ensure_record_size(6)?;
        self.jump_bytes(4)?; // Jump record header
        self.take_u16()
    }

    fn take_i16_record(&mut self) -> GdsReadResult<i16> {
        self.ensure_record_size(6)?;
        self.jump_bytes(4)?; // Jump record header
        self.take_i16()
    }

    fn take_i32_record(&mut self) -> GdsReadResult<i32> {
        self.ensure_record_size(6)?;
        self.jump_bytes(4)?;
        self.take_i32()
    }

    fn take_string_record(&mut self) -> GdsReadResult<String> {
        let size = self.take_record_size()?;
        self.jump_bytes(2)?;
        self.take_string(size - 4)
    }

    fn take_f64_record(&mut self) -> GdsReadResult<f64> {
        self.ensure_record_size(12)?;
        self.jump_bytes(4)?;
        self.take_f64()
    }
}

/// Methods for take value from `reader`
impl<R: Read + Seek> GdsReader<R> {
    fn take_record_size(&mut self) -> GdsReadResult<usize> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        let size = u16::from_be_bytes(buf) as usize;
        if size < 4 {
            return Err(GdsReadError::InvalidRecordSize(size));
        }
        Ok(size)
    }

    fn take_u16(&mut self) -> GdsReadResult<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn take_i16(&mut self) -> GdsReadResult<i16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn take_i32(&mut self) -> GdsReadResult<i32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn take_string(&mut self, len: usize) -> GdsReadResult<String> {
        let mut buf = vec![0u8; len];
        self.reader.read_exact(&mut buf)?;
        while buf.last() == Some(&0) {
            buf.pop();
        }
        let s = String::from_utf8(buf)?;
        Ok(s)
    }

    fn take_f64(&mut self) -> GdsReadResult<f64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
    
        let data = u64::from_be_bytes(buf); 
    
        let sign = (data >> 63) & 0x1;
        let mut exponent = (data >> 56) & 0x7F;
        let mut mantissa = data << 8; 
    
        if mantissa == 0 {
            return Ok(0.0);
        }
    
        exponent = ((exponent as i64 - 64) * 4 + 1023) as u64;
    
        while (mantissa & 0x8000_0000_0000_0000) == 0 {
            mantissa <<= 1;
            exponent -= 1;
        }
    
        mantissa <<= 1;
        exponent -= 1;
    
        let ieee_bits =
            (sign << 63) |
            (exponent << 52) |
            ((mantissa >> 12) & 0x000F_FFFF_FFFF_FFFF);
    
        Ok(f64::from_bits(ieee_bits))
    }

    fn take_datetime(&mut self) -> GdsReadResult<GdsDateTime> {
        let year = self.take_i16()?;
        let month = self.take_i16()?;
        let day = self.take_i16()?;
        let hour = self.take_i16()?;
        let minute = self.take_i16()?;
        let second = self.take_i16()?;

        Ok(GdsDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second
        })
    }
}

impl<R: Read + Seek> GdsReader<R> {
    fn check_record_type(&mut self, tp: GdsRecordType) -> GdsReadResult<bool> {
        let real_tp = self.peek_record_type()?;
        Ok(real_tp == tp)
    }

    fn ensure_record(&mut self, size: usize, tp: GdsRecordType) -> GdsReadResult<()> {
        self.ensure_record_size(size)?;
        self.ensure_record_type(tp)?;
        Ok(())
    }

    fn ensure_record_type(&mut self, tp: GdsRecordType) -> GdsReadResult<()> { 
        let real_tp = self.peek_record_type()?;
        if real_tp == tp {
            Ok(())
        } else {
            Err(GdsReadError::UnexpectRecordType(tp, real_tp))
        }
    }

    fn ensure_record_size(&mut self, size: usize) -> GdsReadResult<()> { 
        let real_size = self.peek_record_size()?;
        if real_size == size {
            Ok(())
        } else {
            Err(GdsReadError::UnexpectRecordSize(size, real_size))
        }
    }

    fn peek_record_size(&mut self) -> GdsReadResult<usize> {
        let bytes = self.peek_bytes::<2>()?;
        Ok(u16::from_be_bytes(bytes) as usize)
    }

    fn peek_record_type(&mut self) -> GdsReadResult<GdsRecordType> {
        let bytes = self.peek_bytes::<4>()?;
        let bytes = [bytes[2], bytes[3]];
        let value = u16::from_be_bytes(bytes);
        match GdsRecordType::from_u16(value) {
            Some(t) => Ok(t),
            None => Err(GdsReadError::UnsupportRecordType(value))
        }
    }

    fn peek_bytes<const L: usize>(&mut self) -> GdsReadResult<[u8; L]> {
        let mut bytes = [0u8; L];
        let buf = self.reader.fill_buf()?;
        if buf.len() < L {
            return Err(GdsReadError::Io(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough bytes to peek")));
        }

        for i in 0..L {
            bytes[i] = buf[i];
        }

        Ok(bytes)
    }

    fn jump_bytes(&mut self, size: i64) -> GdsReadResult<()> {
        self.reader.seek(SeekFrom::Current(size))?;
        Ok(())
    }
}