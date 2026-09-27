use crate::{LefError, LefResult, LefACCurrentDensity, LefAntennaModelRule, LefArraySpacing, LefCutLayer, LefCutSpacingRule, LefCutSpacingTable, LefImplantLayer, LefLayer, LefMastersliceLayer, LefMinimumCutConnection, LefMinimumCutRule, LefMinstepRule, LefMinstepType, LefPropertyValue, LefProtrusionWidth, LefRoutingDirection, LefRoutingLayer, LefSpacingRules, LefSpacingTable, LefSpacingType};
use super::LefTechnologyReader;
use crate::si2;
use super::utils;
use std::{collections::HashMap, os::raw::{c_int, c_void}, str::FromStr};
use paste::paste;

macro_rules! layer_attr_opt {
    ($layer:ident, $si2_obj:ident, $field_name:ident, $attr_name:ident, $as_ty:ty) => {
        paste! {
            if si2::[< lefiLayer_has $attr_name:camel >]($si2_obj) != 0 { 
                $layer.$field_name = Some(si2::[< lefiLayer_ $attr_name >]($si2_obj) as $as_ty);
            }
        }
    };
    
    ($layer:ident, $si2_obj:ident, $field_name:ident, $attr_name:ident) => {
        paste! {
            if si2::[< lefiLayer_has $attr_name:camel >]($si2_obj) != 0 {
                $layer.$field_name = Some(si2::[< lefiLayer_ $attr_name >]($si2_obj));
            }
        }
    };
}

macro_rules! layer_attr {
    ($layer:ident, $si2_obj:ident, $field_name:ident, $attr_name:ident) => {
        paste! {
            if si2::[< lefiLayer_has $attr_name:camel >]($si2_obj) != 0 { 
                $layer.$field_name = si2::[< lefiLayer_ $attr_name >]($si2_obj);
            }
        }
    };

    ($layer:ident, $si2_obj:ident, $field_name:ident, $attr_name:ident, $as_ty:ty) => {
        paste! {
            if si2::[< lefiLayer_has $attr_name:camel >]($si2_obj) != 0 { 
                $layer.$field_name = si2::[< lefiLayer_ $attr_name >]($si2_obj) as $as_ty;
            }
        }
    };
}

impl LefTechnologyReader {
    pub unsafe extern "C" fn read_layer(_: si2::lefrCallbackType_e, obj: *mut si2::lefiLayer, ud: *mut c_void) -> c_int {
        unsafe {
            let reader = &mut *(ud as *mut Self);
            let tpe =  utils::const_c_char_ptr_to_str(si2::lefiLayer_type(obj));
            match tpe {
                "CUT" => {
                    match Self::read_cut_layer(obj) {
                        Ok(layer) => reader.lef.layers.push(LefLayer::Cut(layer)),
                        Err(err) => { 
                            reader.error = Some(err);
                            return 1;
                        }
                    }
                }
                "ROUTING" => {
                    match Self::read_routing_layer(obj) {
                        Ok(layer) => reader.lef.layers.push(LefLayer::Routing(layer)),
                        Err(err) => { 
                            reader.error = Some(err);
                            return 1;
                        }
                    }
                }
                "MASTERSLICE" | "OVERLAP" => {
                    match Self::read_masterslice_layer(obj) {
                        Ok(layer) => reader.lef.layers.push(LefLayer::Masterslice(layer)),
                        Err(err) => { 
                            reader.error = Some(err);
                            return 1;
                        }
                    }
                }
                "IMPLANT" => {
                    match Self::read_implant_layer(obj) {
                        Ok(layer) => reader.lef.layers.push(LefLayer::Implant(layer)),
                        Err(err) => { 
                            reader.error = Some(err);
                            return 1;
                        }
                    }
                }
                _ => {
                    reader.error = Some(LefError::Msg(format!("unsupport layer type: {}", tpe)));
                    return 1;
                }
            }
        }
        0
    }

    unsafe fn read_routing_layer(obj: *mut si2::lefiLayer) -> LefResult<LefRoutingLayer> {
        unsafe {
            let mut layer = LefRoutingLayer::default();
            layer.name = utils::const_c_char_ptr_to_string(si2::lefiLayer_name(obj));

            layer_attr_opt!(layer, obj, mask_num, mask, u32);
            layer_attr!(layer, obj, width, width);
            layer_attr!(layer, obj, cap_multiplier, capMultiplier, u32);
            layer_attr_opt!(layer, obj, diag_width, diagWidth);
            layer_attr_opt!(layer, obj, diag_spacing, diagSpacing);
            layer_attr_opt!(layer, obj, diag_min_edge_length, diagMinEdgeLength);
            layer_attr_opt!(layer, obj, min_area, area);           
            layer_attr_opt!(layer, obj, max_width, maxwidth);      
            layer_attr_opt!(layer, obj, min_width, minwidth);      
            layer_attr_opt!(layer, obj, height, height);           
            layer_attr_opt!(layer, obj, resistance, resistancePerCut);
            layer_attr_opt!(layer, obj, resistance, resistance);
            layer_attr_opt!(layer, obj, capacitance, capacitance); 
            layer_attr_opt!(layer, obj, thickness, thickness);
            layer_attr_opt!(layer, obj, edge_capacitance, edgeCap);
            layer_attr_opt!(layer, obj, shrinkage, shrinkage);
            layer_attr_opt!(layer, obj, minimum_density, minimumDensity);
            layer_attr_opt!(layer, obj, maximum_density, maximumDensity);
            layer_attr_opt!(layer, obj, density_check_step, densityCheckStep);
            layer_attr_opt!(layer, obj, fill_active_spacing, fillActiveSpacing);
            layer_attr_opt!(layer, obj, wire_extension, wireExtension);

            if si2::lefiLayer_hasDirection(obj) != 0 { // DIRECTION
                let direction = utils::const_c_char_ptr_to_str(si2::lefiLayer_direction(obj));
                layer.direction = LefRoutingDirection::from_str(&direction).unwrap();   
            }

            if si2::lefiLayer_hasPitch(obj) != 0 { // PITCH
                let pitch = si2::lefiLayer_pitch(obj);
                layer.pitch = (pitch, pitch);
            } else if si2::lefiLayer_hasXYPitch(obj) != 0 {
                let x_pitch = si2::lefiLayer_pitchX(obj);
                let y_pitch = si2::lefiLayer_pitchY(obj);
                layer.pitch = (x_pitch, y_pitch);
            } 
            
            if si2::lefiLayer_hasDiagPitch(obj) != 0 { // DIAG PITCH
                let pitch = si2::lefiLayer_diagPitch(obj);
                layer.diag_pitch = Some((pitch, pitch));
            } else if si2::lefiLayer_hasXYDiagPitch(obj) != 0 {
                let x_pitch = si2::lefiLayer_diagPitchX(obj);
                let y_pitch = si2::lefiLayer_diagPitchY(obj);
                layer.diag_pitch = Some((x_pitch, y_pitch));
            } 
            
            if si2::lefiLayer_hasOffset(obj) != 0 { // OFFSET
                let offset = si2::lefiLayer_offset(obj);
                layer.offset = Some((offset, offset));
            } else if si2::lefiLayer_hasXYOffset(obj) != 0 {
                let x_offset = si2::lefiLayer_offsetX(obj);
                let y_offset = si2::lefiLayer_offsetY(obj);
                layer.offset = Some((x_offset, y_offset));
            } 

            for index in 0..si2::lefiLayer_numMinSize(obj) {
                layer.min_size.push((si2::lefiLayer_minSizeWidth(obj, index), si2::lefiLayer_minSizeLength(obj, index)));
            }

            if si2::lefiLayer_hasSpacingNumber(obj) != 0 { // SPACING
                for index in 0..si2::lefiLayer_numSpacing(obj) {
                    let min_spacing = si2::lefiLayer_spacing(obj, index);
                    let spacing_type = if si2::lefiLayer_hasSpacingRange(obj, index) != 0 {
                        let min_width = si2::lefiLayer_spacingRangeMin(obj, index);
                        let max_width = si2::lefiLayer_spacingRangeMax(obj, index);
                        let range_type = if si2::lefiLayer_hasSpacingRangeInfluence(obj, index) != 0 {
                            let inf = si2::lefiLayer_spacingRangeInfluence(obj, index);
                            if si2::lefiLayer_hasSpacingRangeInfluenceRange(obj, index) != 0 {
                                Some(crate::LefSpacingRangeType::Influence { influence_length: inf })
                            } else {
                                Some(crate::LefSpacingRangeType::Influence { influence_length: inf })
                            }
                        } else {
                            None
                        };
                        Some(LefSpacingType::Range { min_width, max_width, spacing_range_type: range_type })
                    } else if si2::lefiLayer_hasSpacingEndOfLine(obj, index) != 0{
                        let eol_width = si2::lefiLayer_spacingEolWidth(obj, index);
                        let eol_widthing = si2::lefiLayer_spacingEolWithin(obj, index);
                        Some(LefSpacingType::EndOfLine { eol_width, eol_widthing })
                    } else if si2::lefiLayer_hasSpacingSamenet(obj, index) != 0{
                        let power_ground_only = si2::lefiLayer_hasSpacingSamenetPGonly(obj, index) != 0;
                        Some(LefSpacingType::SameNet { power_ground_only })
                    } else if si2::lefiLayer_hasSpacingNotchLength(obj, index) != 0{
                        let min_notch_length = si2::lefiLayer_spacingNotchLength(obj, index);
                        Some(LefSpacingType::NotchLength { min_notch_length })
                    } else if si2::lefiLayer_hasSpacingEndOfNotchWidth(obj, index) != 0{
                        let end_of_notch_width = si2::lefiLayer_spacingEndOfNotchWidth(obj, index);
                        let min_notch_spacing = si2::lefiLayer_spacingEndOfNotchSpacing(obj, index);
                        let min_notch_length = si2::lefiLayer_spacingEndOfNotchLength(obj, index);
                        Some(LefSpacingType::EndOfNotchWidth { end_of_notch_width, min_notch_spacing, min_notch_length })
                    } else {    
                        None
                    };
                    layer.spacing.push(LefSpacingRules { min_spacing, spacing_type });
                }
            }

            if si2::lefiLayer_numSpacingTable(obj) == 1 { // SPACINGTABLE
                let table = si2::lefiLayer_spacingTable(obj, 0);
                if si2::lefiSpacingTable_isInfluence(table) != 0 {
                    todo!();
                } else if si2::lefiSpacingTable_isParallel(table) != 0 {
                    let parallel = si2::lefiSpacingTable_parallel(table);
                    
                    let parallel_run_lengths = (0..si2::lefiParallel_numLength(parallel))
                        .map(|col| si2::lefiParallel_length(parallel, col))
                        .collect();

                    let widths = (0..si2::lefiParallel_numWidth(parallel))
                        .map(|row| si2::lefiParallel_width(parallel, row))
                        .collect();
                    
                    let spacings: Vec<Vec<_>> = (0..si2::lefiParallel_numWidth(parallel))
                        .map(|row| {
                            (0..si2::lefiParallel_numLength(parallel))
                                .map(|col| si2::lefiParallel_widthSpacing(parallel, row, col))
                                .collect::<Vec<f64>>()
                        })
                        .collect();
                    
                    layer.spacing_table = Some(LefSpacingTable { parallel_run_lengths, widths, spacings });
                }                
            }
    
            // MINSTEP
            for index in 0..si2::lefiLayer_numMinstep(obj) {
                let mut rule = LefMinstepRule::default();
                rule.min_step = si2::lefiLayer_minstep(obj, index);
                if si2::lefiLayer_hasMinstepType(obj, index) != 0 {
                    let t = utils::const_c_char_ptr_to_str(si2::lefiLayer_minstepType(obj, index));
                    rule.min_type = Some(LefMinstepType::from_str(t).unwrap());
                }
                if si2::lefiLayer_hasMinstepLengthsum(obj, index) != 0 {
                    rule.length_sum = Some(si2::lefiLayer_minstepLengthsum(obj, index));
                }
                if si2::lefiLayer_hasMinstepMaxedges(obj, index) != 0 {
                    rule.max_edges = Some(si2::lefiLayer_minstepMaxedges(obj, index) as u32);
                }
                if si2::lefiLayer_hasMinstepMinAdjLength(obj, index) != 0 {
                    rule.min_adj_length = Some(si2::lefiLayer_minstepMinAdjLength(obj, index));
                }
                if si2::lefiLayer_hasMinstepMinBetLength(obj, index) != 0 {
                    rule.min_bet_length = Some(si2::lefiLayer_minstepMinBetLength(obj, index));
                }
                if si2::lefiLayer_hasMinstepXSameCorners(obj, index) != 0 {
                    rule.x_same_corners = true;
                }
                layer.min_step.push(rule);
            }

            // MINIMUMCUT
            for index in 0..si2::lefiLayer_numMinimumcut(obj) {
                let mut rule = LefMinimumCutRule::default();
                rule.num_cuts = si2::lefiLayer_minimumcut(obj, index) as u32;
                rule.min_width = si2::lefiLayer_minimumcutWidth(obj, index);
                if si2::lefiLayer_hasMinimumcutWithin(obj, index) != 0 {
                    rule.within = Some(si2::lefiLayer_minimumcutWithin(obj, index));
                }
                if si2::lefiLayer_hasMinimumcutConnection(obj, index) != 0 {
                    let conn = utils::const_c_char_ptr_to_str(si2::lefiLayer_minimumcutConnection(obj, index));
                    rule.connection = Some(LefMinimumCutConnection::from_str(conn).unwrap());
                }
                if si2::lefiLayer_hasMinimumcutNumCuts(obj, index) != 0 {
                    rule.length_within = Some((si2::lefiLayer_minimumcutLength(obj, index), si2::lefiLayer_minimumcutDistance(obj, index)));
                }
                layer.minimum_cut.push(rule);
            }

            // PROTRUSIONWIDTH
            if si2::lefiLayer_hasProtrusion(obj) != 0 {
                layer.protrusion_width = Some(LefProtrusionWidth {
                    width1: si2::lefiLayer_protrusionWidth1(obj),
                    width2: si2::lefiLayer_protrusionWidth2(obj),
                    length: si2::lefiLayer_protrusionLength(obj),
                });
            }

            // AC CURRENT DENSITY
            if si2::lefiLayer_hasAccurrentDensity(obj) != 0 {
                let mut ac = LefACCurrentDensity::default();
                let density = si2::lefiLayer_accurrent(obj, 0);
                if !density.is_null() {
                    if si2::lefiLayerDensity_hasOneEntry(density) != 0 {
                        ac.peak = si2::lefiLayerDensity_oneEntry(density);
                    } else {
                        // frequency table
                        for fi in 0..si2::lefiLayerDensity_numFrequency(density) {
                            let f = si2::lefiLayerDensity_frequency(density, fi);
                            let t = si2::lefiLayerDensity_tableEntry(density, fi);
                            ac.frequency.push((f, t));
                        }
                        // width table
                        for wi in 0..si2::lefiLayerDensity_numWidths(density) {
                            let w = si2::lefiLayerDensity_width(density, wi);
                            let t = si2::lefiLayerDensity_tableEntry(density, wi);
                            ac.width_table.push((w, t));
                        }
                        // cut area table
                        for ci in 0..si2::lefiLayerDensity_numCutareas(density) {
                            let a = si2::lefiLayerDensity_cutArea(density, ci);
                            let t = si2::lefiLayerDensity_tableEntry(density, ci);
                            ac.cut_area_table.push((a, t));
                        }
                    }
                }
                layer.ac_current_density = Some(ac);
            }

            // DC CURRENT DENSITY
            for index in 0..si2::lefiLayer_numDccurrentDensity(obj) {
                let density = si2::lefiLayer_dccurrent(obj, index);
                if !density.is_null() {
                    if si2::lefiLayerDensity_hasOneEntry(density) != 0 {
                        layer.dc_current_density.push((0.0, si2::lefiLayerDensity_oneEntry(density)));
                    } else {
                        for fi in 0..si2::lefiLayerDensity_numFrequency(density) {
                            let width = si2::lefiLayerDensity_frequency(density, fi);
                            let current = si2::lefiLayerDensity_tableEntry(density, fi);
                            layer.dc_current_density.push((width, current));
                        }
                    }
                }
            }

            // DENSITY CHECK WINDOW
            if si2::lefiLayer_hasDensityCheckWindow(obj) != 0 {
                layer.density_check_window = Some((
                    si2::lefiLayer_densityCheckWindowLength(obj),
                    si2::lefiLayer_densityCheckWindowWidth(obj),
                ));
            }

            // MIN ENCLOSED AREA
            for index in 0..si2::lefiLayer_numMinenclosedarea(obj) {
                let area = si2::lefiLayer_minenclosedarea(obj, index);
                let width = if si2::lefiLayer_hasMinenclosedareaWidth(obj, index) != 0 {
                    Some(si2::lefiLayer_minenclosedareaWidth(obj, index))
                } else {
                    None
                };
                layer.min_enclosed_area.push((area, width));
            }

            // ANTENNA AREA/LENGTH
            if si2::lefiLayer_hasAntennaArea(obj) != 0 {
                layer.antenna_area = Some(si2::lefiLayer_antennaArea(obj));
            }
            if si2::lefiLayer_hasAntennaLength(obj) != 0 {
                layer.antenna_length = Some(si2::lefiLayer_antennaLength(obj));
            }

            // ANTENNA MODELS
            for index in 0..si2::lefiLayer_numAntennaModel(obj) {
                let model = si2::lefiLayer_antennaModel(obj, index);
                if !model.is_null() {
                    let mut rule = LefAntennaModelRule::default();
                    // oxide is a string index, get it from a function
                    rule.oxide = "default".into();
                    if si2::lefiAntennaModel_hasAntennaAreaRatio(model) != 0 {
                        rule.area_ratio = Some(si2::lefiAntennaModel_antennaAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaDiffAreaRatio(model) != 0 {
                        rule.diff_area_ratio = Some(si2::lefiAntennaModel_antennaDiffAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaCumAreaRatio(model) != 0 {
                        rule.cum_area_ratio = Some(si2::lefiAntennaModel_antennaCumAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaCumDiffAreaRatio(model) != 0 {
                        rule.cum_diff_area_ratio = Some(si2::lefiAntennaModel_antennaCumDiffAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaAreaFactor(model) != 0 {
                        rule.area_factor = Some(si2::lefiAntennaModel_antennaAreaFactor(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaSideAreaRatio(model) != 0 {
                        rule.side_area_ratio = Some(si2::lefiAntennaModel_antennaSideAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaDiffSideAreaRatio(model) != 0 {
                        rule.diff_side_area_ratio = Some(si2::lefiAntennaModel_antennaDiffSideAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaCumSideAreaRatio(model) != 0 {
                        rule.cum_side_area_ratio = Some(si2::lefiAntennaModel_antennaCumSideAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaCumDiffSideAreaRatio(model) != 0 {
                        rule.cum_diff_side_area_ratio = Some(si2::lefiAntennaModel_antennaCumDiffSideAreaRatio(model));
                    }
                    if si2::lefiAntennaModel_hasAntennaSideAreaFactor(model) != 0 {
                        rule.side_area_factor = Some(si2::lefiAntennaModel_antennaSideAreaFactor(model));
                    }
                    layer.antenna_models.push(rule);
                }
            }

            Self::read_layer_props(obj, &mut layer.properties);
            Ok(layer)
        }
    }

    unsafe fn read_cut_layer(obj: *mut si2::lefiLayer) -> LefResult<LefCutLayer> {
        unsafe {
            let mut layer = LefCutLayer::default();
            layer.name = utils::const_c_char_ptr_to_string(si2::lefiLayer_name(obj));
            
            layer_attr_opt!(layer, obj, mask_num, mask, u32);
            layer_attr_opt!(layer, obj, width, width);
            layer_attr_opt!(layer, obj, resistance, resistance);

            if si2::lefiLayer_hasSpacingNumber(obj) != 0 {
                for index in 0..si2::lefiLayer_numSpacing(obj) {
                    let spacing = si2::lefiLayer_spacing(obj, index);
                    let center_to_center = si2::lefiLayer_hasSpacingCenterToCenter(obj, index) != 0;
                    let same_net = si2::lefiLayer_hasSpacingSamenet(obj, index) != 0;
                    layer.spacing.push(LefCutSpacingRule {
                        spacing, center_to_center, same_net
                    });
                }
            }

            // ENCLOSURE
            for index in 0..si2::lefiLayer_numEnclosure(obj) {
                let mut rule = crate::LefEnclosureRule::default();
                rule.overhang1 = si2::lefiLayer_enclosureOverhang1(obj, index);
                rule.overhang2 = si2::lefiLayer_enclosureOverhang2(obj, index);
                if si2::lefiLayer_hasEnclosureRule(obj, index) != 0 {
                    let above = si2::lefiLayer_enclosureOverhang1(obj, index);
                    let below = si2::lefiLayer_enclosureOverhang2(obj, index);
                    // `hasEnclosureRule` tells us which sides this applies to
                    // The actual overhangs are the same as overhang1/overhang2
                    rule.above = above != 0.0;
                    rule.below = below != 0.0;
                }
                if si2::lefiLayer_hasEnclosureWidth(obj, index) != 0 {
                    rule.min_width = si2::lefiLayer_enclosureMinWidth(obj, index);
                }
                if si2::lefiLayer_hasEnclosureExceptExtraCut(obj, index) != 0 {
                    rule.except_extracut_within = si2::lefiLayer_enclosureExceptExtraCut(obj, index);
                }
                if si2::lefiLayer_hasEnclosureMinLength(obj, index) != 0 {
                    rule.min_length = si2::lefiLayer_enclosureMinLength(obj, index);
                }
                layer.enclosure.push(rule);
            }

            // PREFER ENCLOSURE
            for index in 0..si2::lefiLayer_numPreferEnclosure(obj) {
                let mut rule = crate::LefEnclosureRule::default();
                rule.overhang1 = si2::lefiLayer_preferEnclosureOverhang1(obj, index);
                rule.overhang2 = si2::lefiLayer_preferEnclosureOverhang2(obj, index);
                if si2::lefiLayer_hasPreferEnclosureRule(obj, index) != 0 {
                    // The rule designation tells us which sides
                }
                if si2::lefiLayer_hasPreferEnclosureWidth(obj, index) != 0 {
                    rule.min_width = si2::lefiLayer_preferEnclosureMinWidth(obj, index);
                }
                layer.prefer_enclosure.push(rule);
            }

            // ARRAY SPACING
            if si2::lefiLayer_hasArraySpacing(obj) != 0 {
                let mut arr = LefArraySpacing::default();
                arr.via_width = si2::lefiLayer_arrayCuts(obj, 0) as f64;
                arr.cut_spacing = si2::lefiLayer_arraySpacing(obj, 0);
                let num_cuts = si2::lefiLayer_numArrayCuts(obj);
                for ci in 0..num_cuts {
                    let cut = si2::lefiLayer_arrayCuts(obj, ci + 1) as i32;
                    let spacing = si2::lefiLayer_arraySpacing(obj, ci);
                    arr.array_cuts.push((cut, spacing));
                }
                layer.array_spacing = Some(arr);
            }

            // CUT SPACING TABLE
            if si2::lefiLayer_hasSpacingTableOrtho(obj) != 0 {
                let num = si2::lefiLayer_numSpacingTable(obj) as usize;
                let mut table = LefCutSpacingTable::default();
                // The ortho spacing table C API doesn't expose individual values easily,
                // so just mark that it exists
                if num > 0 {
                    table.cut_withins = vec![0.0; num];
                    table.ortho_spacings = vec![0.0; num];
                }
                layer.spacing_table = Some(table);
            }

            Self::read_layer_props(obj, &mut layer.properties);
            Ok(layer)
        }
    }

    unsafe fn read_masterslice_layer(obj: *mut si2::lefiLayer) -> LefResult<LefMastersliceLayer> {
        unsafe {
            let mut layer = LefMastersliceLayer::default();
            layer.name = utils::const_c_char_ptr_to_string(si2::lefiLayer_name(obj));
            if si2::lefiLayer_hasMask(obj) != 0 {
                layer.mask_num = Some(si2::lefiLayer_mask(obj) as u32);
            }            

            Self::read_layer_props(obj, &mut layer.properties);
            Ok(layer)
        }
    }

    unsafe fn read_implant_layer(obj: *mut si2::lefiLayer) -> LefResult<LefImplantLayer> {
        unsafe {
            let mut layer = LefImplantLayer::default();
            layer.name = utils::const_c_char_ptr_to_string(si2::lefiLayer_name(obj));
            if si2::lefiLayer_hasWidth(obj) != 0 {
                layer.width = Some(si2::lefiLayer_width(obj));
            }

            Self::read_layer_props(obj, &mut layer.properties);
            Ok(layer)

        }
    }

    unsafe fn read_layer_props(obj: *mut si2::lefiLayer, properties: &mut HashMap<String, LefPropertyValue>) {
        unsafe {
            for index in 0..si2::lefiLayer_numProps(obj) {
                let prop = utils::const_c_char_ptr_to_string(si2::lefiLayer_propName(obj, index));
                if si2::lefiLayer_propIsNumber(obj, index) != 0 {
                    properties.insert(prop, LefPropertyValue::Real(si2::lefiLayer_propNumber(obj, index)));
                } else if si2::lefiLayer_propIsString(obj, index) != 0 {
                    let value = utils::const_c_char_ptr_to_string(si2::lefiLayer_propValue(obj, index));
                    properties.insert(prop, LefPropertyValue::String(value));
                }
            }
        }
    }
}