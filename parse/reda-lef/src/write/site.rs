use crate::si2;
use crate::{LefResult, LefTechnology, LefCellLibrary, LefSite, LefSiteClass, LefSymmetry};
use super::{cstr, check};

impl LefTechnology {
    pub(crate) unsafe fn write_sites(&self) -> LefResult<()> {
        for (name, site) in &self.sites {
            unsafe { write_site(name, site)?; }
        }
        Ok(())
    }
}

impl LefCellLibrary {
    pub(crate) unsafe fn write_sites(&self) -> LefResult<()> {
        for (name, site) in &self.sites {
            unsafe { write_site(name, site)?; }
        }
        Ok(())
    }
}

unsafe fn write_site(name: &str, site: &LefSite) -> LefResult<()> {
    let name_c = cstr(name)?;
    let class_str = match site.class {
        LefSiteClass::CORE => cstr("CORE")?,
        LefSiteClass::PAD => cstr("PAD")?,
    };
    unsafe {
        let sym = build_symmetry_str(&site.symmetry);
        let sym_c = cstr(&sym)?;
        check(si2::lefwSite(name_c.as_ptr(), class_str.as_ptr(), sym_c.as_ptr(), site.size.width, site.size.height))?;

        if !site.row_pattern.is_empty() {
            for (rp_name, rp_orient) in &site.row_pattern {
                let rp_name_c = cstr(rp_name)?;
                let rp_orient_c = cstr(&rp_orient.to_string())?;
                check(si2::lefwSiteRowPatternStr(rp_name_c.as_ptr(), rp_orient_c.as_ptr()))?;
            }
        }

        check(si2::lefwEndSite(name_c.as_ptr()))?;
    }
    Ok(())
}

pub(crate) fn build_symmetry_str(sym: &LefSymmetry) -> String {
    let mut parts = vec![];
    if sym.x { parts.push("X"); }
    if sym.y { parts.push("Y"); }
    if sym.r90 { parts.push("R90"); }
    parts.join(" ")
}
