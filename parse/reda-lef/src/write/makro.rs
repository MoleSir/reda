use crate::si2;
use crate::{LefCellLibrary, LefMacro, LefMacroPin, LefLayerGeometries, LefGeometry, LefResult};
use super::{cstr, check, write_properties};
use super::geometry::write_geometries_in_port;

impl LefCellLibrary {
    pub(crate) unsafe fn write_macros(&self) -> LefResult<()> {
        for (name, makro) in &self.macros {
            unsafe { write_macro(name, makro)?; }
        }
        Ok(())
    }
}

unsafe fn write_macro(name: &str, m: &LefMacro) -> LefResult<()> {
    let name_c = cstr(name)?;
    unsafe {
        check(si2::lefwStartMacro(name_c.as_ptr()))?;

        if let Some(ref class) = m.class {
            let class_s = class.to_string();
            let parts: Vec<&str> = class_s.split_whitespace().collect();
            let primary = cstr(parts.first().copied().unwrap_or(""))?;
            let sub_cstr;
            let sub_ptr = if parts.len() > 1 {
                sub_cstr = cstr(&parts[1..].join(" "))?;
                sub_cstr.as_ptr()
            } else {
                std::ptr::null()
            };
            check(si2::lefwMacroClass(primary.as_ptr(), sub_ptr))?;
        }

        if m.fixed_mask {
            check(si2::lefwMacroFixedMask())?;
        }

        for f in &m.foreign {
            let cell = cstr(&f.cell_name)?;
            let orient = cstr(&f.orient.to_string())?;
            check(si2::lefwMacroForeignStr(cell.as_ptr(), f.position.x, f.position.y, orient.as_ptr()))?;
        }

        if m.origin.x != 0.0 || m.origin.y != 0.0 {
            check(si2::lefwMacroOrigin(m.origin.x, m.origin.y))?;
        }

        if let Some(ref eeq) = m.eeq {
            check(si2::lefwMacroEEQ(cstr(eeq)?.as_ptr()))?;
        }

        if let Some(size) = m.size {
            check(si2::lefwMacroSize(size.width, size.height))?;
        }

        let sym_str = super::site::build_symmetry_str(&m.symmetry);
        if !sym_str.is_empty() {
            check(si2::lefwMacroSymmetry(cstr(&sym_str)?.as_ptr()))?;
        }

        for site_ref in &m.sites {
            let site_name = cstr(&site_ref.name)?;
            if let Some(ref pattern) = site_ref.pattern {
                let orient = cstr(&pattern.site_orient.to_string())?;
                if let Some(ref step) = pattern.step_pattern {
                    check(si2::lefwMacroSitePatternStr(
                        site_name.as_ptr(),
                        pattern.x_origin, pattern.y_origin,
                        orient.as_ptr(),
                        step.x_start as i32, step.y_start as i32,
                        step.x_step, step.y_step,
                    ))?;
                } else {
                    check(si2::lefwMacroSitePatternStr(
                        site_name.as_ptr(),
                        pattern.x_origin, pattern.y_origin,
                        orient.as_ptr(),
                        0, 0, 0.0, 0.0,
                    ))?;
                }
            } else {
                check(si2::lefwMacroSite(site_name.as_ptr()))?;
            }
        }

        for pin in &m.pins {
            write_pin(pin)?;
        }

        for obs in &m.obs {
            write_obs(obs)?;
        }

        for density in &m.density {
            let layer_c = cstr(&density.layer_name)?;
            check(si2::lefwStartMacroDensity(layer_c.as_ptr()))?;
            for rect in &density.rectangles {
                check(si2::lefwMacroDensityLayerRect(
                    rect.lower_left.x, rect.lower_left.y,
                    rect.upper_right.x, rect.upper_right.y, 0.0,
                ))?;
            }
            check(si2::lefwEndMacroDensity())?;
        }

        write_properties(&m.properties)?;
        check(si2::lefwEndMacro(name_c.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_pin(pin: &LefMacroPin) -> LefResult<()> {
    let name_c = cstr(&pin.name)?;
    unsafe {
        check(si2::lefwStartMacroPin(name_c.as_ptr()))?;

        if let Some(ref tr) = pin.taper_rule {
            check(si2::lefwMacroPinTaperRule(cstr(tr)?.as_ptr()))?;
        }
        if let Some(ref dir) = pin.direction {
            check(si2::lefwMacroPinDirection(cstr(&dir.to_string())?.as_ptr()))?;
        }
        if let Some(ref u) = pin.signal_use {
            check(si2::lefwMacroPinUse(cstr(&u.to_string())?.as_ptr()))?;
        }
        if let Some(ref shape) = pin.shape_type {
            check(si2::lefwMacroPinShape(cstr(&shape.to_string())?.as_ptr()))?;
        }
        if let Some(ref mj) = pin.must_join {
            check(si2::lefwMacroPinMustjoin(cstr(mj)?.as_ptr()))?;
        }
        if let Some(ref ne) = pin.net_expr {
            check(si2::lefwMacroPinNetExpr(cstr(ne)?.as_ptr()))?;
        }
        if let Some(ref ss) = pin.supply_sensitivity {
            check(si2::lefwMacroPinSupplySensitivity(cstr(ss)?.as_ptr()))?;
        }
        if let Some(ref gs) = pin.ground_sensitivity {
            check(si2::lefwMacroPinGroundSensitivity(cstr(gs)?.as_ptr()))?;
        }

        for port in &pin.port {
            write_port(port)?;
        }

        check(si2::lefwEndMacroPin(name_c.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_port(geoms: &LefLayerGeometries) -> LefResult<()> {
    let class_str = match &geoms.class {
        Some(c) => cstr(&c.to_string())?,
        None => cstr("NONE")?,
    };
    unsafe {
        check(si2::lefwStartMacroPinPort(class_str.as_ptr()))?;
        write_geometries_in_port(geoms)?;
        check(si2::lefwEndMacroPinPort())?;
    }
    Ok(())
}

unsafe fn write_obs(geoms: &LefLayerGeometries) -> LefResult<()> {
    let layer = cstr(&geoms.layer_name)?;
    unsafe {
        check(si2::lefwStartMacroObs())?;
        if geoms.except_pg_net {
            check(si2::lefwMacroExceptPGNet(layer.as_ptr()))?;
        }
        if let Some(w) = geoms.width {
            check(si2::lefwMacroObsLayerWidth(w))?;
        }

        for g in &geoms.geometries {
            let (num_x, num_y, step_x, step_y) = match g {
                LefGeometry::PathIter((_, iter))
                | LefGeometry::RectIter((_, iter))
                | LefGeometry::PolygonIter((_, iter)) => {
                    (iter.num_x as i32, iter.num_y as i32, iter.space_x, iter.space_y)
                }
                _ => (1, 1, 0.0, 0.0),
            };
            match g {
                LefGeometry::Path(pts) | LefGeometry::PathIter((pts, _)) => {
                    let n = pts.len() as i32;
                    let mut xs: Vec<f64> = pts.iter().map(|p| p.x).collect();
                    let mut ys: Vec<f64> = pts.iter().map(|p| p.y).collect();
                    check(si2::lefwMacroObsLayerPath(n, xs.as_mut_ptr(), ys.as_mut_ptr(), num_x, num_y, step_x, step_y, 0))?;
                }
                LefGeometry::Rect(rect) | LefGeometry::RectIter((rect, _)) => {
                    check(si2::lefwMacroObsLayerRect(rect.lower_left.x, rect.lower_left.y, rect.upper_right.x, rect.upper_right.y, num_x, num_y, step_x, step_y, 0))?;
                }
                LefGeometry::Polygon(pts) | LefGeometry::PolygonIter((pts, _)) => {
                    let n = pts.len() as i32;
                    let mut xs: Vec<f64> = pts.iter().map(|p| p.x).collect();
                    let mut ys: Vec<f64> = pts.iter().map(|p| p.y).collect();
                    check(si2::lefwMacroObsLayerPolygon(n, xs.as_mut_ptr(), ys.as_mut_ptr(), num_x, num_y, step_x, step_y, 0))?;
                }
            }
        }

        for via in &geoms.vias {
            let via_name = cstr(&via.via_name)?;
            let (num_x, num_y, step_x, step_y) = match &via.iter {
                Some(iter) => (iter.num_x as i32, iter.num_y as i32, iter.space_x, iter.space_y),
                None => (1, 1, 0.0, 0.0),
            };
            check(si2::lefwMacroObsVia(via.position.x, via.position.y, via_name.as_ptr(), num_x, num_y, step_x, step_y, 0))?;
        }

        check(si2::lefwEndMacroObs())?;
    }
    Ok(())
}
