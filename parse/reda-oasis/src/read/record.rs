use std::io::Read;
use crate::model::{CellNameRecord, EndRecord, PropNameRecord, StartOffsetFlag, StartRecord, StrictMode, TableOffsetEntry, TableOffsets, TextStringRecord, ValidationScheme};
use super::{read_a_string, read_b_string, read_bytes, read_n_string, read_real, read_u8, read_unsigned_integer, ErrorContext, ReadError, ReadResult};

// =============================================================================================
//                           StartRecord (record 1)
// =============================================================================================

pub fn read_start_record<R: Read>(stream: &mut R) -> ReadResult<StartRecord> {
    let version_string = read_a_string(stream).context("read <version_string>")?;
    let unit = read_real(stream).context("read <unit>")?;
    
    let offset_flag = read_u8(stream).context("read <offset_flag>")?;
    let offset_flag = match offset_flag {
        0 => StartOffsetFlag::InStart,
        1 => StartOffsetFlag::InEnd,
        _ => Err(ReadError::UnsupportOffsetFlag(offset_flag))?
    };
    
    let table_offsets = if offset_flag == StartOffsetFlag::InStart {
        Some(read_table_offsets(stream).context("read [table_offsets]")?)
    } else {
        None
    };

    Ok(StartRecord { version_string, unit, offset_flag, table_offsets })
}

// =============================================================================================
//                           EndRecord (record 2)
// =============================================================================================

pub fn read_end_record<R: Read>(stream: &mut R, offset_flag: StartOffsetFlag) -> ReadResult<EndRecord> {
    let table_offsets = if offset_flag == StartOffsetFlag::InEnd {
        Some(read_table_offsets(stream).context("read [table_offsets]")?)
    } else {
        None
    };

    let padding_string = read_b_string(stream).context("read <padding_string>")?;
    let padding_length = padding_string.0.len();
    let validation_scheme = read_validation_scheme(stream).context("read <validation_scheme>")?;

    let validation_signature: Option<[u8; 4]> = match validation_scheme {
        ValidationScheme::Crc32 | ValidationScheme::Checksum32 => {
            Some(read_bytes(stream).context("read [validation_signature]")?) 
        }
        _ => None,
    };

    Ok(EndRecord { table_offsets, padding_length, validation_scheme, validation_signature })
}

// =============================================================================================
//                           CellNameRecord (record 3 / 4)
// =============================================================================================

pub fn read_implicit_cell_name_record<R: Read>(stream: &mut R) -> ReadResult<CellNameRecord> {
    let cellname_string = read_n_string(stream).context("read <cellname-string>")?;
    Ok(CellNameRecord::Implicit(cellname_string))
}

pub fn read_explicit_cell_name_record<R: Read>(stream: &mut R) -> ReadResult<CellNameRecord> {
    let cellname_string = read_n_string(stream).context("read <cellname-string>")?;
    let reference_number = read_unsigned_integer(stream).context("read <reference-number>")?;
    Ok(CellNameRecord::Explicit(cellname_string, reference_number))
}

// =============================================================================================
//                           TextStringRecord (record 5 / 6)
// =============================================================================================

pub fn read_implicit_text_string_record<R: Read>(stream: &mut R) -> ReadResult<TextStringRecord> {
    let text_string = read_a_string(stream).context("read <text-string>")?;
    Ok(TextStringRecord::Implicit(text_string))
}

pub fn read_explicit_text_string_record<R: Read>(stream: &mut R) -> ReadResult<TextStringRecord> {
    let text_string = read_a_string(stream).context("read <text-string>")?;
    let reference_number = read_unsigned_integer(stream).context("read <reference-number>")?;
    Ok(TextStringRecord::Explicit(text_string, reference_number))
}

// =============================================================================================
//                           PropNameRecord (record 7 / 8)
// =============================================================================================

pub fn read_implicit_prop_name_record<R: Read>(stream: &mut R) -> ReadResult<PropNameRecord> {
    let prop_name = read_n_string(stream).context("read <propname-string>")?;
    Ok(PropNameRecord::Implicit(prop_name))
}

pub fn read_explicit_prop_name_record<R: Read>(stream: &mut R) -> ReadResult<PropNameRecord> {
    let prop_name = read_n_string(stream).context("read <propname-string>")?;
    let reference_number = read_unsigned_integer(stream).context("read <reference-number>")?;
    Ok(PropNameRecord::Explicit(prop_name, reference_number))
}

pub fn read_table_offsets<R: Read>(stream: &mut R) -> ReadResult<TableOffsets> {
    let cell_name = read_table_offset_entry(stream).context("read <cell_name>")?;
    let text_string = read_table_offset_entry(stream).context("read <text_string>")?;
    let prop_name = read_table_offset_entry(stream).context("read <prop_name>")?;
    let prop_string = read_table_offset_entry(stream).context("read <prop_string>")?;
    let layer_name = read_table_offset_entry(stream).context("read <layer_name>")?;
    let xname = read_table_offset_entry(stream).context("read <xname>")?;

    Ok(TableOffsets { cell_name, text_string, prop_name, prop_string, layer_name, xname })
}

pub fn read_table_offset_entry<R: Read>(stream: &mut R) -> ReadResult<TableOffsetEntry> {
    let strict = read_strict_mode(stream).context("read <strict>")?;
    let offset = read_unsigned_integer(stream).context("read <offset>")?;
    Ok(TableOffsetEntry { strict, offset })
}   

pub fn read_strict_mode<R: Read>(stream: &mut R) -> ReadResult<StrictMode> {
    let strict = read_u8(stream).context("read <flag>")?;
    match strict {
        1 => Ok(StrictMode::Strict),
        0 => Ok(StrictMode::NonStrict),
        _ => Err(ReadError::UnsupportStrictMode(strict))
    }
}

pub fn read_validation_scheme<R: Read>(stream: &mut R) -> ReadResult<ValidationScheme> {
    let value = read_u8(stream).context("read <validation_scheme>")?;
    match value {
        0 => Ok(ValidationScheme::None),
        1 => Ok(ValidationScheme::Crc32),
        2 => Ok(ValidationScheme::Checksum32),
        _ => Err(ReadError::UnsupportValidationScheme(value))
    }
}