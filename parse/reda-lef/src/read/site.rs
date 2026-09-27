use crate::{LefOrient, LefSite, LefSiteClass, LefSize, LefSymmetry};
use super::{LefCellLibraryReader, LefTechnologyReader};
use crate::si2;
use super::utils;
use std::{os::raw::{c_int, c_void}, str::FromStr};

impl LefTechnologyReader {
    pub unsafe extern "C" fn read_site(_: si2::lefrCallbackType_e, obj: *mut si2::lefiSite, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            // si2::lefiSite_numSites(obj)
            let mut site = crate::LefSite::default();
            do_read_site(obj, &mut site);
            reader.lef.sites.insert(site.name.clone(), site);
        }
        0
    }
}

impl LefCellLibraryReader {
    pub unsafe extern "C" fn read_site(_: si2::lefrCallbackType_e, obj: *mut si2::lefiSite, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            // si2::lefiSite_numSites(obj)
            let mut site = crate::LefSite::default();
            do_read_site(obj, &mut site);
            reader.lef.sites.insert(site.name.clone(), site);
        }
        0
    }
}

pub unsafe extern "C" fn do_read_site(obj: *mut si2::lefiSite, site: &mut LefSite) {
    unsafe {
        site.name = utils::const_c_char_ptr_to_string(si2::lefiSite_name(obj));
        if si2::lefiSite_hasSize(obj) != 0 {
            site.size = LefSize::new(si2::lefiSite_sizeX(obj), si2::lefiSite_sizeY(obj));
        }
        if si2::lefiSite_hasClass(obj) != 0 {
            let class = utils::const_c_char_ptr_to_str(si2::lefiSite_siteClass(obj));
            site.class = LefSiteClass::from_str(class).unwrap();
        }

        let x = si2::lefiSite_hasXSymmetry(obj) != 0;
        let y = si2::lefiSite_hasYSymmetry(obj) != 0;
        let r90 = si2::lefiSite_has90Symmetry(obj) != 0;
        site.symmetry = LefSymmetry { x, y, r90 };

        // ROWPATTERN
        if si2::lefiSite_hasRowPattern(obj) != 0 {
            for index in 0..si2::lefiSite_numSites(obj) {
                let name = utils::const_c_char_ptr_to_string(si2::lefiSite_siteName(obj, index));
                let orient = LefOrient::from_str(
                    utils::const_c_char_ptr_to_str(si2::lefiSite_siteOrientStr(obj, index))
                ).unwrap_or_default();
                site.row_pattern.push((name, orient));
            }
        }
    }
}