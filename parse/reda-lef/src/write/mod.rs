mod layer;
mod via;
mod makro;
mod site;
mod geometry;

use std::ffi::CString;
use std::path::Path;

use crate::si2;
use crate::{LefCellLibrary, LefError, LefResult, LefTechnology};

fn check(ret: i32) -> LefResult<()> {
    if ret == 0 { Ok(()) } else { Err(LefError::Write(ret)) }
}

fn cstr(s: &str) -> LefResult<CString> {
    CString::new(s).map_err(|e| LefError::Nul(e))
}

unsafe fn open_for_write(path: &str) -> LefResult<(*mut libc::FILE, *mut si2::FILE)> {
    let c_path = CString::new(path)?;
    let c_mode = CString::new("w").unwrap();
    unsafe {
        let fp = libc::fopen(c_path.as_ptr(), c_mode.as_ptr());
        if fp.is_null() {
            return Err(LefError::Msg(format!("failed to open file: {}", path)));
        }
        let si2_fp = fp as *mut si2::FILE;
        Ok((fp, si2_fp))
    }
}

impl LefTechnology {
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> LefResult<()> {
        let path = path.as_ref().to_str().ok_or_else(|| LefError::Msg("invalid path".into()))?;
        unsafe { self.write_inner(path) }
    }

    unsafe fn write_inner(&self, path: &str) -> LefResult<()> {
        let (fp, si2_fp) = unsafe { open_for_write(path)? };
        unsafe {
            check(si2::lefwInit(si2_fp))?;
            self.write_header()?;
            self.write_units()?;
            self.write_manufacturing_grid()?;
            self.write_layers()?;
            self.write_vias()?;
            self.write_sites()?;
            self.write_via_rules()?;
            check(si2::lefwEnd())?;
            libc::fclose(fp);
        }
        Ok(())
    }

    unsafe fn write_header(&self) -> LefResult<()> {
        if let Some(v) = self.version {
            let major = v as i32;
            let minor = ((v - major as f64) * 10.0) as i32;
            unsafe { check(si2::lefwVersion(major, minor))?; }
        }
        let bb = format!("{}{}", self.busbitchars.0, self.busbitchars.1);
        unsafe { check(si2::lefwBusBitChars(cstr(&bb)?.as_ptr()))?; }
        unsafe { check(si2::lefwDividerChar(cstr(&self.dividerchar.to_string())?.as_ptr()))?; }
        unsafe { check(si2::lefwClearanceMeasure(cstr(&self.clearance_measure.to_string())?.as_ptr()))?; }
        Ok(())
    }

    unsafe fn write_manufacturing_grid(&self) -> LefResult<()> {
        if let Some(grid) = self.manufacturing_grid {
            unsafe { check(si2::lefwManufacturingGrid(grid))?; }
        }
        Ok(())
    }

    unsafe fn write_units(&self) -> LefResult<()> {
        let u = &self.units;
        unsafe {
            check(si2::lefwStartUnits())?;
            check(si2::lefwUnits(
                u.time_ns as f64, u.capacitance_pf as f64,
                u.resistance_ohms as f64, u.power_mw as f64,
                u.current_ma as f64, u.voltage_v as f64,
                u.database_microns as f64,
            ))?;
            if u.frequency_mega_hz > 0 {
                check(si2::lefwUnitsFrequency(u.frequency_mega_hz as f64))?;
            }
            check(si2::lefwEndUnits())?;
        }
        Ok(())
    }
}

impl LefCellLibrary {
    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> LefResult<()> {
        let path = path.as_ref().to_str().ok_or_else(|| LefError::Msg("invalid path".into()))?;
        unsafe { self.write_inner(path) }
    }

    unsafe fn write_inner(&self, path: &str) -> LefResult<()> {
        let (fp, si2_fp) = unsafe { open_for_write(path)? };
        unsafe {
            check(si2::lefwInit(si2_fp))?;
            self.write_header()?;
            self.write_sites()?;
            self.write_vias()?;
            self.write_macros()?;
            check(si2::lefwEnd())?;
            libc::fclose(fp);
        }
        Ok(())
    }

    unsafe fn write_header(&self) -> LefResult<()> {
        if let Some(v) = self.version {
            let major = v as i32;
            let minor = ((v - major as f64) * 10.0) as i32;
            unsafe { check(si2::lefwVersion(major, minor))?; }
        }
        let bb = format!("{}{}", self.busbitchars.0, self.busbitchars.1);
        unsafe { check(si2::lefwBusBitChars(cstr(&bb)?.as_ptr()))?; }
        unsafe { check(si2::lefwDividerChar(cstr(&self.dividerchar.to_string())?.as_ptr()))?; }
        Ok(())
    }
}

unsafe fn write_properties(
    properties: &std::collections::HashMap<String, crate::LefPropertyValue>,
) -> LefResult<()> {
    for (name, val) in properties {
        let name = cstr(name)?;
        match val {
            crate::LefPropertyValue::Int(v) => unsafe { check(si2::lefwIntProperty(name.as_ptr(), *v))?; },
            crate::LefPropertyValue::Real(v) => unsafe { check(si2::lefwRealProperty(name.as_ptr(), *v))?; },
            crate::LefPropertyValue::String(v) => {
                let vs = cstr(v)?;
                unsafe { check(si2::lefwStringProperty(name.as_ptr(), vs.as_ptr()))?; }
            },
        }
    }
    Ok(())
}
