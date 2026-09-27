use crate::si2;
use crate::{LefLayerGeometries, LefGeometry, LefResult};
use super::{cstr, check};

/// Write geometries inside a MACRO PIN PORT block.
pub(crate) unsafe fn write_geometries_in_port(geoms: &LefLayerGeometries) -> LefResult<()> {
    let layer = cstr(&geoms.layer_name)?;
    let spacing = match &geoms.spacing_or_designrule_width {
        Some(crate::LefSpacingOrDesignRuleWidth::MinSpacing(s)) => *s,
        _ => 0.0,
    };
    unsafe {
        // lefwMacroPinPortLayer and lefwMacroPinPortDesignRuleWidth are mutually exclusive
        if let Some(crate::LefSpacingOrDesignRuleWidth::DesignRuleWidth(w)) = &geoms.spacing_or_designrule_width {
            check(si2::lefwMacroPinPortDesignRuleWidth(layer.as_ptr(), *w))?;
        } else {
            check(si2::lefwMacroPinPortLayer(layer.as_ptr(), spacing))?;
        }

        if let Some(w) = geoms.width {
            check(si2::lefwMacroPinPortLayerWidth(w))?;
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
                    check(si2::lefwMacroPinPortLayerPath(n, xs.as_mut_ptr(), ys.as_mut_ptr(), num_x, num_y, step_x, step_y, 0))?;
                }
                LefGeometry::Rect(rect) | LefGeometry::RectIter((rect, _)) => {
                    check(si2::lefwMacroPinPortLayerRect(rect.lower_left.x, rect.lower_left.y, rect.upper_right.x, rect.upper_right.y, num_x, num_y, step_x, step_y, 0))?;
                }
                LefGeometry::Polygon(pts) | LefGeometry::PolygonIter((pts, _)) => {
                    let n = pts.len() as i32;
                    let mut xs: Vec<f64> = pts.iter().map(|p| p.x).collect();
                    let mut ys: Vec<f64> = pts.iter().map(|p| p.y).collect();
                    check(si2::lefwMacroPinPortLayerPolygon(n, xs.as_mut_ptr(), ys.as_mut_ptr(), num_x, num_y, step_x, step_y, 0))?;
                }
            }
        }

        for via in &geoms.vias {
            let via_name = cstr(&via.via_name)?;
            let (num_x, num_y, step_x, step_y) = match &via.iter {
                Some(iter) => (iter.num_x as i32, iter.num_y as i32, iter.space_x, iter.space_y),
                None => (1, 1, 0.0, 0.0),
            };
            check(si2::lefwMacroPinPortVia(via.position.x, via.position.y, via_name.as_ptr(), num_x, num_y, step_x, step_y, 0))?;
        }
    }
    Ok(())
}
