use reda_geometry::shape::Rect;

use crate::{read::LefError, LefFixVia, LefGeneratedVia, LefPropertyValue, LefVia, LefViaBotTop, LefViaEnclosure, LefViaLayers, LefViaRowCol, LefViaRule, LefViaRuleWidth, LefViaShape, LefXy};
use super::{LefCellLibraryReader, LefTechnologyReader};
use crate::si2;
use super::utils;
use std::os::raw::c_int;

impl LefTechnologyReader {
    pub unsafe extern "C" fn read_via(_: si2::lefrCallbackType_e, obj: *mut si2::lefiVia, ud: *mut ::std::os::raw::c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            let via_name = utils::const_c_char_ptr_to_string(si2::lefiVia_name(obj));
            let via = if si2::lefiVia_hasViaRule(obj) != 0 {
                let mut via = LefGeneratedVia::default();
                read_generated_via(obj, &mut via);
                LefVia::Generated(via)
            } else {
                let mut via = LefFixVia::default();
                read_fix_via(obj, &mut via);
                LefVia::Fix(via)
            };
            reader.lef.vias.insert(via_name, via);
        }
        
        0
    }

    pub unsafe extern "C" fn read_viarule(_: si2::lefrCallbackType_e, obj: *mut si2::lefiViaRule, ud: *mut ::std::os::raw::c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            let via_name = utils::const_c_char_ptr_to_string(si2::lefiViaRule_name(obj));
            let mut via_rule = LefViaRule::default();

            if si2::lefiViaRule_hasGenerate(obj) == 0 {
                reader.error = Some(LefError::UnsupportFixedViaRule(via_name));
                return 1;
            }
            
            via_rule.is_default = si2::lefiViaRule_hasDefault(obj) != 0;
            via_rule.rule_name = via_name.clone();

            assert_eq!(si2::lefiViaRule_numLayers(obj), 3);
            let bottom_layer = si2::lefiViaRule_layer(obj, 0);
            let top_layer = si2::lefiViaRule_layer(obj, 1);
            let cut_layer = si2::lefiViaRule_layer(obj,2);

            // LAYERS
            let bottom_layer_name = utils::const_c_char_ptr_to_string(si2::lefiViaRuleLayer_name(bottom_layer));
            let top_layer_name = utils::const_c_char_ptr_to_string(si2::lefiViaRuleLayer_name(top_layer));
            let cut_layer_name = utils::const_c_char_ptr_to_string(si2::lefiViaRuleLayer_name(cut_layer));
            via_rule.layers = LefViaLayers::new(bottom_layer_name, cut_layer_name, top_layer_name);

            // ENCLOSURE
            if si2::lefiViaRuleLayer_hasEnclosure(bottom_layer) != 0 {
                via_rule.bottom_enclosure = LefViaEnclosure::new(si2::lefiViaRuleLayer_enclosureOverhang1(bottom_layer), si2::lefiViaRuleLayer_enclosureOverhang2(bottom_layer));
            }
            if si2::lefiViaRuleLayer_hasEnclosure(top_layer) != 0 {
                via_rule.top_enclosure = LefViaEnclosure::new(si2::lefiViaRuleLayer_enclosureOverhang1(top_layer), si2::lefiViaRuleLayer_enclosureOverhang2(top_layer));
            }

            // WIDTH
            if si2::lefiViaRuleLayer_hasWidth(bottom_layer) != 0 {
                via_rule.bottom_width = Some(LefViaRuleWidth::new(si2::lefiViaRuleLayer_widthMin(bottom_layer), si2::lefiViaRuleLayer_widthMax(bottom_layer)));
            }
            if si2::lefiViaRuleLayer_hasWidth(top_layer) != 0 {
                via_rule.top_width = Some(LefViaRuleWidth::new(si2::lefiViaRuleLayer_widthMin(bottom_layer), si2::lefiViaRuleLayer_widthMax(bottom_layer)));
            }

            // RECT
            if si2::lefiViaRuleLayer_hasRect(cut_layer) != 0 {
                via_rule.rect.lower_left =  (si2::lefiViaRuleLayer_xl(cut_layer), si2::lefiViaRuleLayer_yl(cut_layer)).into();
                via_rule.rect.upper_right = (si2::lefiViaRuleLayer_xh(cut_layer), si2::lefiViaRuleLayer_yh(cut_layer)).into();
            }
            
            // SPACING
            if si2::lefiViaRuleLayer_hasSpacing(cut_layer) != 0 {
                via_rule.spacing.x = si2::lefiViaRuleLayer_spacingStepX(cut_layer);
                via_rule.spacing.y = si2::lefiViaRuleLayer_spacingStepY(cut_layer);
            }
            
            reader.lef.via_rules.insert(via_name, via_rule);
        }
        
        0
    }
}

impl LefCellLibraryReader {
    pub unsafe extern "C" fn read_via(_: si2::lefrCallbackType_e, obj: *mut si2::lefiVia, ud: *mut ::std::os::raw::c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);

            let via_name = utils::const_c_char_ptr_to_string(si2::lefiVia_name(obj));
            let via = if si2::lefiVia_hasViaRule(obj) != 0 {
                let mut via = LefGeneratedVia::default();
                read_generated_via(obj, &mut via);
                LefVia::Generated(via)
            } else {
                let mut via = LefFixVia::default();
                read_fix_via(obj, &mut via);
                LefVia::Fix(via)
            };
            reader.lef.vias.insert(via_name, via);
        }
        
        0
    }
}

pub unsafe extern "C" fn read_generated_via(obj: *mut si2::lefiVia, via: &mut LefGeneratedVia) {
    unsafe {
        // LAYERS
        via.viarule_name = utils::const_c_char_ptr_to_string(si2::lefiVia_viaRuleName(obj));
        via.layers.bottom = utils::const_c_char_ptr_to_string(si2::lefiVia_botMetalLayer(obj));
        via.layers.cut = utils::const_c_char_ptr_to_string(si2::lefiVia_cutLayer(obj));
        via.layers.top = utils::const_c_char_ptr_to_string(si2::lefiVia_topMetalLayer(obj));

        via.is_default = si2::lefiVia_hasDefault(obj) != 0;

        // CUTSIZE
        via.cut_size = LefXy::new(si2::lefiVia_xCutSize(obj), si2::lefiVia_yCutSize(obj));

        // CUTSPACING
        via.cut_spacing = LefXy::new(si2::lefiVia_xCutSpacing(obj), si2::lefiVia_yCutSpacing(obj));
    
        // ENCLOSURE
        via.enclosure = LefViaBotTop::new(
            si2::lefiVia_xBotEnc(obj), 
            si2::lefiVia_yBotEnc(obj),
            si2::lefiVia_xTopEnc(obj),
            si2::lefiVia_yTopEnc(obj),
        );

        // ROWCOL
        if si2::lefiVia_hasRowCol(obj) != 0 {
            via.rowcol = LefViaRowCol::new(si2::lefiVia_numCutRows(obj) as usize, si2::lefiVia_numCutCols(obj) as usize);
        }

        // ORIGIN
        if si2::lefiVia_hasOrigin(obj) != 0 {
            via.origin = LefXy::new(si2::lefiVia_xOffset(obj), si2::lefiVia_yOffset(obj));
        }

        // OFFSET
        if si2::lefiVia_hasOffset(obj) != 0 {
            via.offset = LefViaBotTop::new(
                si2::lefiVia_xBotOffset(obj), 
                si2::lefiVia_yBotOffset(obj),
                si2::lefiVia_xTopOffset(obj),
                si2::lefiVia_yTopOffset(obj),
            );
        }

        // PATTERN
        if si2::lefiVia_hasCutPattern(obj) != 0 {
            via.pattern = Some(utils::const_c_char_ptr_to_string(si2::lefiVia_cutPattern(obj)));
        }

        for index in 0..si2::lefiVia_numProperties(obj) {
            let prop = utils::const_c_char_ptr_to_string(si2::lefiVia_propName(obj, index));
            if si2::lefiVia_propIsNumber(obj, index) != 0 {
                via.properties.insert(prop, LefPropertyValue::Real(si2::lefiVia_propNumber(obj, index)));
            } else if si2::lefiVia_propIsString(obj, index) != 0 {
                let value = utils::const_c_char_ptr_to_string(si2::lefiVia_propValue(obj, index));
                via.properties.insert(prop, LefPropertyValue::String(value));
            }
        }
    }
}

pub unsafe extern "C" fn read_fix_via(obj: *mut si2::lefiVia, via: &mut LefFixVia) {
    unsafe {
        via.is_default = si2::lefiVia_hasDefault(obj) != 0;

        // RESISTANCE
        if si2::lefiVia_hasResistance(obj) != 0 {
            via.resistance = Some(si2::lefiVia_resistance(obj));
        }

        // LAYER
        for l in 0..si2::lefiVia_numLayers(obj) {
            let layer_name = utils::const_c_char_ptr_to_string(si2::lefiVia_layerName(obj, l));
            let mut shapes = vec![];
            for r in 0..si2::lefiVia_numRects(obj, l) {
                let xl = si2::lefiVia_xl(obj, l, r);
                let yl = si2::lefiVia_yl(obj, l, r);
                let xh = si2::lefiVia_xh(obj, l, r);
                let yh = si2::lefiVia_yh(obj, l, r);
                shapes.push(LefViaShape::Rect(Rect::new((xl, yl), (xh, yh))));
            }
            for p in 0..si2::lefiVia_numPolygons(obj, l) {
                let poly = si2::lefiVia_getPolygon(obj, l, p);
                let points = (0..poly.numPoints as usize)
                    .map(|i| (*poly.x.add(i), *poly.y.add(i)))
                    .collect();
                shapes.push(LefViaShape::Polygon(points));
            }
            via.geometry.insert(layer_name, shapes);
        }

        for index in 0..si2::lefiVia_numProperties(obj) {
            let prop = utils::const_c_char_ptr_to_string(si2::lefiVia_propName(obj, index));
            if si2::lefiVia_propIsNumber(obj, index) != 0 {
                via.properties.insert(prop, LefPropertyValue::Real(si2::lefiVia_propNumber(obj, index)));
            } else if si2::lefiVia_propIsString(obj, index) != 0 {
                let value = utils::const_c_char_ptr_to_string(si2::lefiVia_propValue(obj, index));
                via.properties.insert(prop, LefPropertyValue::String(value));
            }
        }
    }
}
