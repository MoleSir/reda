use crate::si2;
use crate::{LefCutLayer, LefLayer, LefRoutingLayer, LefSpacingRules, LefSpacingType, LefSpacingRangeType, LefResult, LefTechnology};
use super::{cstr, check, write_properties};

impl LefTechnology {
    pub(crate) unsafe fn write_layers(&self) -> LefResult<()> {
        for layer in &self.layers {
            match layer {
                LefLayer::Masterslice(l) => unsafe { write_masterslice(l)? },
                LefLayer::Cut(l) => unsafe { write_cut(l)? },
                LefLayer::Routing(l) => unsafe { write_routing(l)? },
                LefLayer::Implant(l) => unsafe { write_implant(l)? },
            }
        }
        Ok(())
    }
}

unsafe fn write_masterslice(l: &crate::LefMastersliceLayer) -> LefResult<()> {
    let name = cstr(&l.name)?;
    let type_str = cstr("MASTERSLICE")?;
    unsafe {
        check(si2::lefwStartLayer(name.as_ptr(), type_str.as_ptr()))?;
        if let Some(m) = l.mask_num {
            check(si2::lefwLayerMask(m as i32))?;
        }
        write_properties(&l.properties)?;
        check(si2::lefwEndLayer(name.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_implant(l: &crate::LefImplantLayer) -> LefResult<()> {
    let name = cstr(&l.name)?;
    let type_str = cstr("IMPLANT")?;
    unsafe {
        check(si2::lefwStartLayer(name.as_ptr(), type_str.as_ptr()))?;
        if let Some(w) = l.width {
            check(si2::lefwLayerWidth(w))?;
        }
        write_properties(&l.properties)?;
        check(si2::lefwEndLayer(name.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_cut(l: &LefCutLayer) -> LefResult<()> {
    let name = cstr(&l.name)?;
    let type_str = cstr("CUT")?;
    unsafe {
        check(si2::lefwStartLayer(name.as_ptr(), type_str.as_ptr()))?;
        if let Some(m) = l.mask_num {
            check(si2::lefwLayerMask(m as i32))?;
        }
        if let Some(w) = l.width {
            check(si2::lefwLayerWidth(w))?;
        }
        for s in &l.spacing {
            check(si2::lefwLayerCutSpacing(s.spacing))?;
            if s.center_to_center {
                check(si2::lefwLayerCutSpacingCenterToCenter())?;
            }
            if s.same_net {
                check(si2::lefwLayerCutSpacingSameNet())?;
            }
            check(si2::lefwLayerCutSpacingEnd())?;
        }
        for e in &l.enclosure {
            write_enclosure(e)?;
        }
        for e in &l.prefer_enclosure {
            write_prefer_enclosure(e)?;
        }
        if let Some(res) = l.resistance {
            check(si2::lefwLayerResistancePerCut(res))?;
        }
        write_properties(&l.properties)?;
        check(si2::lefwEndLayer(name.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_enclosure(e: &crate::LefEnclosureRule) -> LefResult<()> {
    let loc = if e.above && !e.below {
        cstr("ABOVE")?
    } else if e.below && !e.above {
        cstr("BELOW")?
    } else {
        cstr("")?
    };
    unsafe {
        if e.except_extracut_within > 0.0 {
            check(si2::lefwLayerEnclosureWidth(loc.as_ptr(), e.overhang1, e.overhang2, e.min_width, e.except_extracut_within))?;
        } else if e.min_width > 0.0 {
            check(si2::lefwLayerEnclosureWidth(loc.as_ptr(), e.overhang1, e.overhang2, e.min_width, 0.0))?;
        } else {
            check(si2::lefwLayerEnclosure(loc.as_ptr(), e.overhang1, e.overhang2, e.min_width))?;
        }
    }
    Ok(())
}

unsafe fn write_prefer_enclosure(e: &crate::LefEnclosureRule) -> LefResult<()> {
    let loc = if e.above && !e.below {
        cstr("ABOVE")?
    } else if e.below && !e.above {
        cstr("BELOW")?
    } else {
        cstr("")?
    };
    unsafe {
        check(si2::lefwLayerPreferEnclosure(loc.as_ptr(), e.overhang1, e.overhang2, e.min_width))?;
    }
    Ok(())
}

unsafe fn write_routing(l: &LefRoutingLayer) -> LefResult<()> {
    let name = cstr(&l.name)?;
    unsafe {
        check(si2::lefwStartLayerRouting(name.as_ptr()))?;
        check(si2::lefwLayerRouting(cstr(&l.direction.to_string())?.as_ptr(), l.width))?;

        if l.pitch.0 == l.pitch.1 {
            check(si2::lefwLayerRoutingPitch(l.pitch.0))?;
        } else {
            check(si2::lefwLayerRoutingPitchXYDistance(l.pitch.0, l.pitch.1))?;
        }

        if let Some((dx, dy)) = l.diag_pitch {
            if dx == dy { check(si2::lefwLayerRoutingDiagPitch(dx))?; }
            else { check(si2::lefwLayerRoutingDiagPitchXYDistance(dx, dy))?; }
        }
        if let Some(dw) = l.diag_width { check(si2::lefwLayerRoutingDiagWidth(dw))?; }
        if let Some(ds) = l.diag_spacing { check(si2::lefwLayerRoutingDiagSpacing(ds))?; }
        if let Some(dml) = l.diag_min_edge_length { check(si2::lefwLayerRoutingDiagMinEdgeLength(dml))?; }
        if let Some((ox, oy)) = l.offset {
            if ox == oy { check(si2::lefwLayerRoutingOffset(ox))?; }
            else { check(si2::lefwLayerRoutingOffsetXYDistance(ox, oy))?; }
        }
        if let Some(area) = l.min_area { check(si2::lefwLayerRoutingArea(area))?; }

        if !l.min_size.is_empty() {
            let mut widths: Vec<f64> = l.min_size.iter().map(|(w, _)| *w).collect();
            let mut lengths: Vec<f64> = l.min_size.iter().map(|(_, l)| *l).collect();
            check(si2::lefwLayerRoutingMinsize(widths.len() as i32, widths.as_mut_ptr(), lengths.as_mut_ptr()))?;
        }

        for s in &l.spacing {
            write_routing_spacing(s)?;
        }

        if let Some(we) = l.wire_extension { check(si2::lefwLayerRoutingWireExtension(we))?; }
        if let Some(mw) = l.max_width { check(si2::lefwLayerRoutingMaxwidth(mw))?; }
        if let Some(mw) = l.min_width { check(si2::lefwLayerRoutingMinwidth(mw))?; }

        if let Some(res) = l.resistance {
            check(si2::lefwLayerRoutingResistance(cstr(&format!("{}", res))?.as_ptr()))?;
        }
        if let Some(cap) = l.capacitance {
            check(si2::lefwLayerRoutingCapacitance(cstr(&format!("{}", cap))?.as_ptr()))?;
        }
        if let Some(h) = l.height { check(si2::lefwLayerRoutingHeight(h))?; }
        if let Some(t) = l.thickness { check(si2::lefwLayerRoutingThickness(t))?; }
        if let Some(s) = l.shrinkage { check(si2::lefwLayerRoutingShrinkage(s))?; }
        if l.cap_multiplier != 1 { check(si2::lefwLayerRoutingCapMultiplier(l.cap_multiplier as f64))?; }
        if let Some(ec) = l.edge_capacitance { check(si2::lefwLayerRoutingEdgeCap(ec))?; }
        if let Some(aa) = l.antenna_area { check(si2::lefwLayerRoutingAntennaArea(aa))?; }
        if let Some(al) = l.antenna_length { check(si2::lefwLayerRoutingAntennaLength(al))?; }

        write_properties(&l.properties)?;
        check(si2::lefwEndLayerRouting(name.as_ptr()))?;
    }
    Ok(())
}

unsafe fn write_routing_spacing(s: &LefSpacingRules) -> LefResult<()> {
    unsafe {
        check(si2::lefwLayerRoutingSpacing(s.min_spacing))?;
        match &s.spacing_type {
            Some(LefSpacingType::Range { min_width, max_width, spacing_range_type }) => {
                check(si2::lefwLayerRoutingSpacingRange(*min_width, *max_width))?;
                match spacing_range_type {
                    Some(LefSpacingRangeType::UseLengthThreshold) => {
                        check(si2::lefwLayerRoutingSpacingRangeUseLengthThreshold())?;
                    }
                    Some(LefSpacingRangeType::Influence { influence_length }) => {
                        check(si2::lefwLayerRoutingSpacingRangeInfluence(*influence_length, 0.0, 0.0))?;
                    }
                    None => {}
                }
            }
            Some(LefSpacingType::EndOfLine { eol_width, eol_widthing }) => {
                check(si2::lefwLayerRoutingSpacingEndOfLine(*eol_width, *eol_widthing))?;
            }
            Some(LefSpacingType::SameNet { power_ground_only }) => {
                check(si2::lefwLayerRoutingSpacingSameNet(if *power_ground_only { 1 } else { 0 }))?;
            }
            Some(LefSpacingType::NotchLength { min_notch_length }) => {
                check(si2::lefwLayerRoutingSpacingNotchLength(*min_notch_length))?;
            }
            Some(LefSpacingType::EndOfNotchWidth { end_of_notch_width, min_notch_spacing, min_notch_length }) => {
                check(si2::lefwLayerRoutingSpacingEndOfNotchWidth(*end_of_notch_width, *min_notch_spacing, *min_notch_length))?;
            }
            None => {}
        }
    }
    Ok(())
}
