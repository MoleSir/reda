use reda_geometry::shape::Rect;

use crate::{LefError, LefGeometry, LefGeometryClass, LefGeometryIter, LefLayerGeometries, LefMacro, LefMacroForegin, LefMacroPin, LefOrient, LefPinDirection, LefPinShape, LefPropertyValue, LefSignalUse, LefSiteRefPattern, LefSiteRefStepPattern, LefSiteReference, LefSize, LefSpacingOrDesignRuleWidth, LefSymmetry, LefViaUse};
use super::LefCellLibraryReader;
use crate::si2;
use super::utils;
use std::{os::raw::{c_int, c_void}, str::FromStr};

impl LefCellLibraryReader {
    pub unsafe extern "C" fn read_macro(_: si2::lefrCallbackType_e, obj: *mut si2::lefiSite, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            let mut makcro = LefMacro::default();
            makcro.name = utils::const_c_char_ptr_to_string(si2::lefiMacro_name(obj));

            // CLASS
            if si2::lefiMacro_hasClass(obj) != 0 {
                match FromStr::from_str(utils::const_c_char_ptr_to_str(si2::lefiMacro_macroClass(obj))) {
                    Ok(class) => makcro.class = Some(class),
                    Err(err) => {
                        reader.error = Some(LefError::Msg(err));
                        return 1;
                    }
                }
            }

            // FIXEDMASK
            makcro.fixed_mask = si2::lefiMacro_isFixedMask(obj) != 0;

            // FOREIGN
            if si2::lefiMacro_hasForeign(obj) != 0 {
                for index in 0..si2::lefiMacro_numForeigns(obj) {
                    let name = utils::const_c_char_ptr_to_string(si2::lefiMacro_foreignName(obj, index));
                    let (pos, orient) = if si2::lefiMacro_hasForeignPoint(obj, index) != 0 {
                        let pos = (si2::lefiMacro_foreignX(obj, index), si2::lefiMacro_foreignY(obj, index));
                        if si2::lefiMacro_hasForeignOrient(obj, index) != 0 {
                            (pos, LefOrient::from_str(utils::const_c_char_ptr_to_str(si2::lefiMacro_foreignOrientStr(obj, index))).unwrap())
                        } else {
                            (pos, LefOrient::default())
                        }
                    } else {
                        ((0.0, 0.0), LefOrient::default()) 
                    };
                    
                    makcro.foreign.push(LefMacroForegin::new(name, pos.into(), orient));
                }
            }

            // ORIGIN
            if si2::lefiMacro_hasOrigin(obj) != 0 {
                makcro.origin = (si2::lefiMacro_originX(obj), si2::lefiMacro_originY(obj)).into();
            }

            // EEQ
            if si2::lefiMacro_hasEEQ(obj) != 0 {
                makcro.eeq = Some(utils::const_c_char_ptr_to_string(si2::lefiMacro_EEQ(obj)));
            }
            
            // SIZE
            if si2::lefiMacro_hasSize(obj) != 0 {
                makcro.size = Some(LefSize::new(si2::lefiMacro_sizeX(obj), si2::lefiMacro_sizeY(obj)));
            }

            // SYMMETRY
            makcro.symmetry = LefSymmetry {
                x: si2::lefiMacro_hasXSymmetry(obj) != 0,
                y: si2::lefiMacro_hasYSymmetry(obj) != 0,
                r90: si2::lefiMacro_has90Symmetry(obj) != 0,
            };

            // SITE
            if si2::lefiMacro_hasSiteName(obj) != 0 {
                let mut site = LefSiteReference::default();
                site.name = utils::const_c_char_ptr_to_string(si2::lefiMacro_siteName(obj));
                makcro.sites.push(site);
            }
            for index in 0..si2::lefiMacro_numSitePattern(obj) {
                let pattern_obj = si2::lefiMacro_sitePattern(obj, index);

                let mut site = LefSiteReference::default();
                let mut pattern = LefSiteRefPattern::default();

                site.name = utils::const_c_char_ptr_to_string(si2::lefiSitePattern_name(pattern_obj));
                pattern.x_origin = si2::lefiSitePattern_x(pattern_obj);
                pattern.y_origin = si2::lefiSitePattern_y(pattern_obj);
                pattern.site_orient = LefOrient::from_str(utils::const_c_char_ptr_to_str(si2::lefiSitePattern_orientStr(pattern_obj))).unwrap();

                if si2::lefiSitePattern_hasStepPattern(pattern_obj) != 0 {
                    pattern.step_pattern = Some(LefSiteRefStepPattern {
                        x_start: si2::lefiSitePattern_xStart(pattern_obj),
                        y_start: si2::lefiSitePattern_yStart(pattern_obj),
                        x_step: si2::lefiSitePattern_xStep(pattern_obj),
                        y_step: si2::lefiSitePattern_yStep(pattern_obj),
                    })
                }

                site.pattern = Some(pattern);
                makcro.sites.push(site);
            }
   
            // PIN
            makcro.pins = reader.take_pins();
            
            // OBS
            makcro.obs = reader.take_geometries();

            // DENSITY
            makcro.density = reader.take_density();

            // PROPERTY
            for index in 0..si2::lefiMacro_numProperties(obj) {
                let name = utils::const_c_char_ptr_to_string(si2::lefiMacro_propName(obj, index));
                // match si2::lefiMacro_prop
                let value = if si2::lefiMacro_propValue(obj, index).is_null() {
                    LefPropertyValue::Real(si2::lefiMacro_propNum(obj, index))
                } else {
                    LefPropertyValue::String(utils::const_c_char_ptr_to_string(si2::lefiMacro_propValue(obj, index)))
                };
                makcro.properties.insert(name, value);
            }

            reader.lef.macros.insert(makcro.name.clone(), makcro);   
        }
        0
    }

    pub unsafe extern "C" fn read_pin(_: si2::lefrCallbackType_e, obj: *mut si2::lefiPin, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            
            let mut pin = LefMacroPin::default();
            pin.name = utils::const_c_char_ptr_to_string(si2::lefiPin_name(obj));  

            if si2::lefiPin_hasDirection(obj) != 0 {
                let dir = LefPinDirection::from_str(&utils::const_c_char_ptr_to_str(si2::lefiPin_direction(obj))).unwrap();
                pin.direction = Some(dir);
            }

            if si2::lefiPin_hasUse(obj) != 0 {
                let uuse =  utils::const_c_char_ptr_to_str(si2::lefiPin_use(obj));
                let uuse = LefSignalUse::from_str(&uuse).unwrap();
                pin.signal_use = Some(uuse);
            }

            if si2::lefiPin_hasShape(obj) != 0 {
                let shape =  utils::const_c_char_ptr_to_str(si2::lefiPin_shape(obj));
                let shape = LefPinShape::from_str(&shape).unwrap();
                pin.shape_type = Some(shape);
            }

            if si2::lefiPin_hasMustjoin(obj) != 0 {
                pin.must_join = Some(utils::const_c_char_ptr_to_string(si2::lefiPin_mustjoin(obj)));
            }

            if si2::lefiPin_hasTaperRule(obj) != 0 {
                pin.taper_rule = Some(utils::const_c_char_ptr_to_string(si2::lefiPin_taperRule(obj)));
            }

            if si2::lefiPin_hasNetExpr(obj) != 0 {
                pin.net_expr = Some(utils::const_c_char_ptr_to_string(si2::lefiPin_netExpr(obj)));
            }

            if si2::lefiPin_hasSupplySensitivity(obj) != 0 {
                pin.supply_sensitivity = Some(utils::const_c_char_ptr_to_string(si2::lefiPin_supplySensitivity(obj)));
            }

            if si2::lefiPin_hasGroundSensitivity(obj) != 0 {
                pin.ground_sensitivity = Some(utils::const_c_char_ptr_to_string(si2::lefiPin_groundSensitivity(obj)));
            }

            for index in 0..si2::lefiPin_numPorts(obj) {
                let port = si2::lefiPin_port(obj, index);
                pin.port = Self::read_geometries(port);
            }

            reader.pins.push(pin);
        }
        0
    }

    pub unsafe extern "C" fn read_obs(_: si2::lefrCallbackType_e, obj: *mut si2::lefiObstruction, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            let obj = si2::lefiObstruction_geometries(obj);
            let geometries = Self::read_geometries(obj);   
            reader.geometries = geometries;                 
        }
        0
    }

    unsafe fn read_geometries(obj: *const si2::lefiGeometries) -> Vec<LefLayerGeometries> {
        unsafe {
            let mut all_geometries: Vec<LefLayerGeometries> = vec![];
            for index in 0..si2::lefiGeometries_numItems(obj) {
                let mut geometries = LefLayerGeometries::default();
                geometries.except_pg_net = si2::lefiGeometries_hasLayerExceptPgNet(obj, index) != 0;
                geometries.spacing_or_designrule_width = {
                    let spacing = si2::lefiGeometries_getLayerMinSpacing(obj, index);
                    let rule_width = si2::lefiGeometries_getLayerRuleWidth(obj, index);
                    if spacing > 0.0 {
                        Some(LefSpacingOrDesignRuleWidth::MinSpacing(spacing))
                    } else if rule_width > 0.0 {
                        Some(LefSpacingOrDesignRuleWidth::DesignRuleWidth(rule_width))
                    } else {
                        None
                    }
                };

                match si2::lefiGeometries_itemType(obj, index) {
                    si2::lefiGeomEnum_lefiGeomClassE => {
                        let class_str = utils::const_c_char_ptr_to_str(si2::lefiGeometries_getClass(obj, index));
                        if let Ok(cls) = LefGeometryClass::from_str(class_str) {
                            geometries.class = Some(cls);
                        }
                    }
                    si2::lefiGeomEnum_lefiGeomLayerE => {
                        geometries.layer_name = utils::const_c_char_ptr_to_string(si2::lefiGeometries_getLayer(obj, index));
                    }
                    si2::lefiGeomEnum_lefiGeomWidthE => {
                        geometries.width = Some(si2::lefiGeometries_getWidth(obj, index));
                    }
                    si2::lefiGeomEnum_lefiGeomPathE => {
                        let path = si2::lefiGeometries_getPath(obj, index);
                        let path = &*path;
                        let mut points = vec![];
                        for j in 0..path.numPoints as usize {
                            points.push((*path.x.add(j), *path.y.add(j)));
                        }
                        geometries.geometries.push(LefGeometry::Path(points.into()));
                    }
                    si2::lefiGeomEnum_lefiGeomPathIterE => {
                        let path_iter = si2::lefiGeometries_getPathIter(obj, index);
                        let path_iter = &*path_iter;
                        let mut points = vec![];
                        for j in 0..path_iter.numPoints as usize {
                            points.push((*path_iter.x.add(j), *path_iter.y.add(j)));
                        }
                        let iter = LefGeometryIter::new(path_iter.xStart as usize, path_iter.yStart as usize , path_iter.xStep, path_iter.yStep);
                        geometries.geometries.push(LefGeometry::PathIter((points.into(), iter)));
                        // for i in 0..path_iter.xStart as usize {
                        //     for j in 0..path_iter.yStart as usize {
                        //         let mut new_points = vec![];
                        //         for p in points.iter() {
                        //             let new_x = p.0 + i as f64 * path_iter.xStep;
                        //             let new_y = p.1 + j as f64 * path_iter.yStep;
                        //             new_points.push((new_x, new_y));
                        //         }
                        //         geometries.geometries.push(LefGeometry::Path(new_points.into()));
                        //     }
                        // }
                    }
                    si2::lefiGeomEnum_lefiGeomRectE => {
                        let rect = si2::lefiGeometries_getRect(obj, index);
                        let rect = &*rect;
                        geometries.geometries.push(LefGeometry::Rect(Rect::new((rect.xl, rect.yl), (rect.xh, rect.yh))));
                    }
                    si2::lefiGeomEnum_lefiGeomRectIterE => {
                        let rect_iter = si2::lefiGeometries_getRectIter(obj, index);
                        let rect_iter = &*rect_iter;
                        let rect = Rect::new((rect_iter.xl, rect_iter.yl), (rect_iter.xh, rect_iter.yh));
                        let iter = LefGeometryIter::new(rect_iter.xStart as usize, rect_iter.yStart as usize , rect_iter.xStep, rect_iter.yStep);
                        geometries.geometries.push(LefGeometry::RectIter((rect, iter)));
                        // for i in 0..rect_iter.xStart as usize {
                        //     for j in 0..rect_iter.yStart as usize {
                        //         let xl = rect.lower_left.x + i as f64 * rect_iter.xStep;
                        //         let yl = rect.lower_left.y + j as f64 * rect_iter.yStep;
                        //         let xh = rect.upper_right.x + i as f64 * rect_iter.xStep;
                        //         let yh = rect.upper_right.y + j as f64 * rect_iter.yStep;
                        //         geometries.geometries.push(LefGeometry::Rect(Rect::new((xl, yl), (xh, yh))));
                        //     }
                        // }
                    }
                    si2::lefiGeomEnum_lefiGeomPolygonE => {
                        let polygon = si2::lefiGeometries_getPolygon(obj, index);
                        let polygon = &*polygon;
                        let mut points = vec![];
                        for j in 0..polygon.numPoints as usize {
                            points.push((*polygon.x.add(j), *polygon.y.add(j)));
                        }
                        geometries.geometries.push(LefGeometry::Polygon(points.into()));
                    }
                    si2::lefiGeomEnum_lefiGeomPolygonIterE => {
                        let polygon_iter = si2::lefiGeometries_getPolygonIter(obj, index);
                        let polygon_iter = &*polygon_iter;
                        let mut points = vec![];
                        for j in 0..polygon_iter.numPoints as usize {
                            points.push((*polygon_iter.x.add(j), *polygon_iter.y.add(j)));
                        }
                        let iter = LefGeometryIter::new(polygon_iter.xStart as usize, polygon_iter.yStart as usize , polygon_iter.xStep, polygon_iter.yStep);
                        geometries.geometries.push(LefGeometry::PolygonIter((points.into(), iter)));
                        // let mut points = vec![];
                        // for j in 0..polygon_iter.numPoints as usize {
                        //     points.push((*polygon_iter.x.add(j), *polygon_iter.y.add(j)));
                        // }
                        // for i in 0..polygon_iter.xStart as usize {
                        //     for j in 0..polygon_iter.yStart as usize {
                        //         let mut new_points = vec![];
                        //         for p in points.iter() {
                        //             let new_x = p.0 + i as f64 * polygon_iter.xStep;
                        //             let new_y = p.1 + j as f64 * polygon_iter.yStep;
                        //             new_points.push((new_x, new_y));
                        //         }
                        //         geometries.geometries.push(LefGeometry::Polygon(new_points.into()));
                        //     }
                        // }
                    }
                    si2::lefiGeomEnum_lefiGeomViaE => {
                        let via = si2::lefiGeometries_getVia(obj, index);
                        let via = &*via;
                        let via = LefViaUse::new(
                            (via.x, via.y).into(), 
                            utils::const_c_char_ptr_to_string(via.name),
                            None,
                        );
                        geometries.vias.push(via);
                    }
                    si2::lefiGeomEnum_lefiGeomViaIterE => {
                        let via_iter = si2::lefiGeometries_getViaIter(obj, index);
                        let via_iter = &*via_iter;
                        let iter = LefGeometryIter::new(via_iter.xStart as usize, via_iter.yStart as usize , via_iter.xStep, via_iter.yStep);
                        let via = LefViaUse::new(
                            (via_iter.x, via_iter.y).into(), 
                            utils::const_c_char_ptr_to_string(via_iter.name),
                            Some(iter),
                        );
                        geometries.vias.push(via);
                    }
                    _ => panic!(),
                };

                all_geometries.push(geometries);
            }
            
            all_geometries
        }
    }
}