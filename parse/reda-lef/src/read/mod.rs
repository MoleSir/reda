pub(crate) mod utils;
mod layer;
mod via;
mod site;
mod makro;

use std::str::FromStr;
use std::sync::RwLock;
use std::path::Path;
use crate::si2;
use crate::LefError;
use crate::LefResult;
use reda_geometry::shape::Rect;
use crate::LefCellLibrary;
use crate::LefClearanceMeasure;
use crate::LefLayerGeometries;
use crate::LefMacroDensity;
use crate::LefMacroPin;
use crate::LefPropertyDataType;
use crate::LefPropertyDefinition;
use crate::LefPropertyType;
use crate::LefTechnology;
use std::os::raw::{c_void, c_int, c_char};
use std::sync::LazyLock;

impl LefTechnology {
    pub fn load_file<P: AsRef<Path>>(path: P) -> LefResult<Self> {
        let reader = LefTechnologyReader::new();
        unsafe { reader.load_file_inner(path.as_ref()) }
    }
}

impl LefCellLibrary {
    pub fn load_file<P: AsRef<Path>>(path: P) -> LefResult<Self> {
        let reader = LefCellLibraryReader::new();
        unsafe { reader.load_file_inner(path.as_ref()) }
    }
}

static ERROR_MESSAGE: LazyLock<RwLock<String>> = LazyLock::new(|| {
    RwLock::new(String::from("hello"))
});

unsafe extern "C" fn log(msg: *const ::std::os::raw::c_char) {
    let msg = unsafe { utils::const_c_char_ptr_to_string(msg) };
    let mut locked = ERROR_MESSAGE.write().unwrap();
    *locked = msg;  
}

//=====================================================================
//                    Technology Reader
//=====================================================================

pub struct LefTechnologyReader {
    lef: LefTechnology,
    error: Option<LefError>,
}

impl LefTechnologyReader {
    fn new() -> Self {
        Self { lef: Default::default(), error: None }
    }

    unsafe fn load_file_inner(mut self, path: &Path) -> LefResult<LefTechnology> {
        let path = path.to_str().unwrap();
        
        ERROR_MESSAGE.write().unwrap().clear();
        unsafe { 
            si2::lefrInit(); 
            si2::lefrSetVersionCbk(Some(Self::read_version));
            si2::lefrSetBusBitCharsCbk(Some(Self::read_busbitchars));
            si2::lefrSetDividerCharCbk(Some(Self::read_dividerchar));
            si2::lefrSetUnitsCbk(Some(Self::read_units));
            si2::lefrSetManufacturingCbk(Some(Self::read_manufacturing_grid));
            si2::lefrSetClearanceMeasureCbk(Some(Self::read_clearance_measure));
            si2::lefrSetSiteCbk(Some(Self::read_site));
            si2::lefrSetLayerCbk(Some(Self::read_layer));
            si2::lefrSetViaCbk(Some(Self::read_via));
            si2::lefrSetViaRuleCbk(Some(Self::read_viarule));
            si2::lefrSetPropCbk(Some(Self::read_prop));
            si2::lefrSetLogFunction(Some(log));

            let self_ptr = &mut self as *mut Self as *mut c_void;

            let (fp, fp_for_lefr) = utils::open_c_file(path, "r");
            let ret = si2::lefrRead(fp_for_lefr, path.as_ptr() as *const std::os::raw::c_char, self_ptr);
            if ret != 0 && self.error.is_none() {
                self.error = Some(LefError::Si2(ERROR_MESSAGE.read().unwrap().clone()));
                ERROR_MESSAGE.write().unwrap().clear();
            }

            si2::lefrReleaseNResetMemory();

            libc::fclose(fp);
        };

        match self.error {
            None => Ok(self.lef),
            Some(err) => Err(err),
        }
    }

    unsafe extern "C" fn read_version(_: si2::lefrCallbackType_e, version: f64, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            reader.lef.version = Some(version);
        }
        0
    }

    unsafe extern "C" fn read_busbitchars(_: si2::lefrCallbackType_e, raw: *const c_char, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            // let busbitchars = utils::const_c_char_ptr_to_cstr(raw);
            reader.lef.busbitchars = ((*raw) as u8 as char, *raw.add(1) as u8 as char);
        }
        0
    }

    unsafe extern "C" fn read_dividerchar(_: si2::lefrCallbackType_e, raw: *const c_char, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            // let dividerchar = utils::const_c_char_ptr_to_cstr(raw);
            reader.lef.dividerchar = (*raw) as u8 as char;
        }
        0
    }

    unsafe extern "C" fn read_units(_: si2::lefrCallbackType_e, obj: *mut si2::lefiUnits, ud: *mut c_void) -> c_int {
        unsafe  {
            let reader = &mut *(ud as *mut Self);
            // NOTE: F64？
            if si2::lefiUnits_hasTime(obj) != 0 {
                reader.lef.units.time_ns = si2::lefiUnits_time(obj) as u64;   
            }
            if si2::lefiUnits_hasCapacitance(obj) != 0 {
                reader.lef.units.capacitance_pf = si2::lefiUnits_capacitance(obj) as u64;
            }
            if si2::lefiUnits_hasResistance(obj) != 0 {
                reader.lef.units.resistance_ohms = si2::lefiUnits_resistance(obj) as u64;
            }
            if si2::lefiUnits_hasPower(obj) != 0 {
                reader.lef.units.power_mw = si2::lefiUnits_power(obj) as u64;
            }
            if si2::lefiUnits_hasCurrent(obj) != 0 {
                reader.lef.units.current_ma = si2::lefiUnits_current(obj) as u64;
            }
            if si2::lefiUnits_hasVoltage(obj) != 0 {
                reader.lef.units.voltage_v = si2::lefiUnits_voltage(obj) as u64;
            }
            if si2::lefiUnits_hasDatabase(obj) != 0 {
                reader.lef.units.database_microns = si2::lefiUnits_databaseNumber(obj) as u64;
            }
            if si2::lefiUnits_hasFrequency(obj) != 0 {
                reader.lef.units.frequency_mega_hz = si2::lefiUnits_frequency(obj) as u64;
            }
        }
        0
    }

    unsafe extern "C" fn read_manufacturing_grid(_: si2::lefrCallbackType_e, number: f64, ud: *mut c_void) -> c_int {
        unsafe  {
            let reader = &mut *(ud as *mut Self);
            reader.lef.manufacturing_grid = Some(number);
        }
        0
    } 

    unsafe extern "C" fn read_clearance_measure(_: si2::lefrCallbackType_e, string: *const c_char, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            reader.lef.clearance_measure = LefClearanceMeasure::from_str(&utils::const_c_char_ptr_to_str(string)).unwrap();
        }
        0
    }

    unsafe extern "C" fn read_prop(_: si2::lefrCallbackType_e, obj: *mut si2::lefiProp, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            let prop_type = LefPropertyType::from_str(utils::const_c_char_ptr_to_str(si2::lefiProp_propType(obj))).unwrap();
            let prop_name = utils::const_c_char_ptr_to_string(si2::lefiProp_propName(obj));
            let data_type = match si2::lefiProp_dataType(obj) as u8 as char {
                'S' => LefPropertyDataType::String,
                'I' => LefPropertyDataType::Integer,
                'R' => LefPropertyDataType::Real,
                _ => panic!(),
            };
            let prop_def = LefPropertyDefinition {
                prop_type, name: prop_name, data_type,
            };

            reader.lef.properties_definitions.push(prop_def);
        }
        0
    }
}

//=====================================================================
//                    Cell Library Reader
//=====================================================================

pub struct LefCellLibraryReader {
    lef: LefCellLibrary,
    error: Option<LefError>,
    pins: Vec<LefMacroPin>,
    geometries: Vec<LefLayerGeometries>,
    density: Vec<crate::LefMacroDensity>,
}

impl LefCellLibraryReader {
    fn new() -> Self {
        Self { lef: Default::default(), error: None, pins: vec![], geometries: vec![], density: vec![] }
    }

    fn take_pins(&mut self) -> Vec<LefMacroPin> {
        std::mem::take(&mut self.pins)
    }

    fn take_geometries(&mut self) -> Vec<LefLayerGeometries> {
        std::mem::take(&mut self.geometries)
    }

    fn take_density(&mut self) -> Vec<crate::LefMacroDensity> {
        std::mem::take(&mut self.density)
    }

    unsafe fn load_file_inner(mut self, path: &Path) -> LefResult<LefCellLibrary> {
        let path = path.to_str().unwrap();
        
        ERROR_MESSAGE.write().unwrap().clear();
        unsafe { 
            si2::lefrInit(); 
            si2::lefrSetVersionCbk(Some(Self::read_version));
            si2::lefrSetBusBitCharsCbk(Some(Self::read_busbitchars));
            si2::lefrSetDividerCharCbk(Some(Self::read_dividerchar));
            si2::lefrSetMacroCbk(Some(Self::read_macro));
            si2::lefrSetSiteCbk(Some(Self::read_site));
            si2::lefrSetPinCbk(Some(Self::read_pin));
            si2::lefrSetViaCbk(Some(Self::read_via));
            si2::lefrSetObstructionCbk(Some(Self::read_obs));
            si2::lefrSetDensityCbk(Some(Self::read_density));
            si2::lefrSetLogFunction(Some(log));

            let self_ptr = &mut self as *mut Self as *mut c_void;

            let (fp, fp_for_lefr) = utils::open_c_file(path, "r");
            let ret = si2::lefrRead(fp_for_lefr, path.as_ptr() as *const std::os::raw::c_char, self_ptr);
            if ret != 0 && self.error.is_none() {
                self.error = Some(LefError::Si2(ERROR_MESSAGE.read().unwrap().clone()));
                ERROR_MESSAGE.write().unwrap().clear();
            }

            si2::lefrReleaseNResetMemory();

            libc::fclose(fp);
        };

        match self.error {
            None => Ok(self.lef),
            Some(err) => Err(err),
        }
    }

    unsafe extern "C" fn read_version(_: si2::lefrCallbackType_e, version: f64, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            reader.lef.version = Some(version);
        }
        0
    }

    unsafe extern "C" fn read_busbitchars(_: si2::lefrCallbackType_e, raw: *const c_char, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            // let busbitchars = utils::const_c_char_ptr_to_cstr(raw);
            reader.lef.busbitchars = ((*raw) as u8 as char, *raw.add(1) as u8 as char);
        }
        0
    }

    unsafe extern "C" fn read_dividerchar(_: si2::lefrCallbackType_e, raw: *const c_char, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            // let dividerchar = utils::const_c_char_ptr_to_cstr(raw);
            reader.lef.dividerchar = (*raw) as u8 as char;
        }
        0
    }

    unsafe extern "C" fn read_density(_: si2::lefrCallbackType_e, obj: *mut si2::lefiDensity, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            for li in 0..si2::lefiDensity_numLayer(obj) {
                let layer_name = utils::const_c_char_ptr_to_string(si2::lefiDensity_layerName(obj, li));
                let mut rectangles = vec![];
                for ri in 0..si2::lefiDensity_numRects(obj, li) {
                    let r = si2::lefiDensity_getRect(obj, li, ri);
                    rectangles.push(Rect::new((r.xl, r.yl), (r.xh, r.yh)));
                }
                reader.density.push(LefMacroDensity { layer_name, rectangles });
            }
        }
        0
    }
}

