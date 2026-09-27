use crate::si2;
use crate::{LefResult, LefTechnology, LefCellLibrary, LefVia, LefViaShape, LefViaRule};
use super::{cstr, check, write_properties};

impl LefTechnology {
    pub(crate) unsafe fn write_vias(&self) -> LefResult<()> {
        for (name, via) in &self.vias {
            unsafe { write_via(name, via)?; }
        }
        Ok(())
    }

    pub(crate) unsafe fn write_via_rules(&self) -> LefResult<()> {
        for (name, rule) in &self.via_rules {
            unsafe { write_via_rule(name, rule)?; }
        }
        Ok(())
    }
}

impl LefCellLibrary {
    pub(crate) unsafe fn write_vias(&self) -> LefResult<()> {
        for (name, via) in &self.vias {
            unsafe { write_via(name, via)?; }
        }
        Ok(())
    }
}

unsafe fn write_via(name: &str, via: &LefVia) -> LefResult<()> {
    let name_c = cstr(name)?;
    let (is_default, _) = match via {
        LefVia::Generated(g) => (g.is_default, true),
        LefVia::Fix(f) => (f.is_default, false),
    };
    let default_c = if is_default { cstr("DEFAULT")? } else { cstr("")? };

    unsafe {
        check(si2::lefwStartVia(name_c.as_ptr(), default_c.as_ptr()))?;
        match via {
            LefVia::Generated(g) => {
                check(si2::lefwViaViarule(
                    cstr(&g.viarule_name)?.as_ptr(),
                    g.cut_size.x, g.cut_size.y,
                    cstr(&g.layers.bottom)?.as_ptr(),
                    cstr(&g.layers.cut)?.as_ptr(),
                    cstr(&g.layers.top)?.as_ptr(),
                    g.cut_spacing.x, g.cut_spacing.y,
                    g.enclosure.x_bottom, g.enclosure.y_bottom,
                    g.enclosure.x_top, g.enclosure.y_top,
                ))?;
                if g.rowcol.num_cut_rows > 1 || g.rowcol.num_cut_cols > 1 {
                    check(si2::lefwViaViaruleRowCol(g.rowcol.num_cut_rows as i32, g.rowcol.num_cut_cols as i32))?;
                }
                if g.origin.x != 0.0 || g.origin.y != 0.0 {
                    check(si2::lefwViaViaruleOrigin(g.origin.x, g.origin.y))?;
                }
                if g.offset.x_bottom != 0.0 || g.offset.y_bottom != 0.0
                    || g.offset.x_top != 0.0 || g.offset.y_top != 0.0
                {
                    check(si2::lefwViaViaruleOffset(
                        g.offset.x_bottom, g.offset.y_bottom,
                        g.offset.x_top, g.offset.y_top,
                    ))?;
                }
                if let Some(ref pattern) = g.pattern {
                    check(si2::lefwViaViarulePattern(cstr(pattern)?.as_ptr()))?;
                }
                write_properties(&g.properties)?;
            }
            LefVia::Fix(f) => {
                if let Some(res) = f.resistance {
                    check(si2::lefwViaResistance(res))?;
                }
                for (layer, shapes) in &f.geometry {
                    let layer_c = cstr(layer)?;
                    check(si2::lefwViaLayer(layer_c.as_ptr()))?;
                    for shape in shapes {
                        match shape {
                            LefViaShape::Rect(r) => {
                                check(si2::lefwViaLayerRect(
                                    r.lower_left.x, r.lower_left.y,
                                    r.upper_right.x, r.upper_right.y, 0,
                                ))?;
                            }
                            LefViaShape::Polygon(pts) => {
                                let n = pts.len() as i32;
                                let mut xs: Vec<f64> = pts.iter().map(|(x, _)| *x).collect();
                                let mut ys: Vec<f64> = pts.iter().map(|(_, y)| *y).collect();
                                check(si2::lefwViaLayerPolygon(n, xs.as_mut_ptr(), ys.as_mut_ptr(), 0))?;
                            }
                        }
                    }
                }
                write_properties(&f.properties)?;
            }
        }
        check(si2::lefwEndVia(name_c.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_via_rule(name: &str, rule: &LefViaRule) -> LefResult<()> {
    let name_c = cstr(name)?;
    unsafe {
        check(si2::lefwStartViaRuleGen(name_c.as_ptr()))?;
        if rule.is_default {
            check(si2::lefwViaRuleGenDefault())?;
        }
        // bottom layer
        check(si2::lefwViaRuleGenLayer(
            cstr(&rule.layers.bottom)?.as_ptr(), cstr("")?.as_ptr(),
            rule.bottom_width.as_ref().map(|w| w.min).unwrap_or(0.0),
            rule.bottom_width.as_ref().map(|w| w.max).unwrap_or(0.0),
            rule.bottom_enclosure.overhang1,
            rule.bottom_enclosure.overhang2,
        ))?;
        // top layer
        check(si2::lefwViaRuleGenLayer(
            cstr(&rule.layers.top)?.as_ptr(), cstr("")?.as_ptr(),
            rule.top_width.as_ref().map(|w| w.min).unwrap_or(0.0),
            rule.top_width.as_ref().map(|w| w.max).unwrap_or(0.0),
            rule.top_enclosure.overhang1,
            rule.top_enclosure.overhang2,
        ))?;
        // cut layer
        check(si2::lefwViaRuleGenLayer3(
            cstr(&rule.layers.cut)?.as_ptr(),
            rule.rect.lower_left.x, rule.rect.lower_left.y,
            rule.rect.upper_right.x, rule.rect.upper_right.y,
            rule.spacing.x, rule.spacing.y,
            rule.resistance.unwrap_or(0.0),
        ))?;
        check(si2::lefwEndViaRuleGen(name_c.as_ptr()))?;
    }
    Ok(())
}
