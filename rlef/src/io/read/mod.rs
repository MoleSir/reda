#[allow(unused)]
mod base;
mod error;

use base::{float, identifier, qstring, unsigned_int, ws};
use nom::{
    branch::alt, 
    bytes::complete::tag, 
    combinator::opt, 
    error::{VerboseError, VerboseErrorKind}, 
    multi::many0, 
    sequence::{delimited, tuple}, 
    Err, Parser
};
use crate::{
    Lef58TrimmedMetal, 
    Lef58Type, 
    LefCutLayer, 
    LefCutLayerBuilder, 
    LefCutSpacing, 
    LefCutSpacingConstraint, 
    LefEnclosure, 
    LefEnclosureCondition, 
    LefImplantLayer, 
    LefImplantLayerBuilder, 
    LefImplantSpacing, 
    LefImplantSpacingBuilder, 
    LefLayer, 
    LefPitch, 
    LefRoutingDirection, 
    LefRoutingLayer, 
    LefRoutingLayerBuilder, 
    LefRoutingSpacing, 
    LefSpecialLayer, 
    LefSpecialLayerBuilder, 
    LefSpecialLayerType, 
    LefTechLibrary, 
    LefTechLibraryBuilder, 
    LefUnits, 
    LefUseMinSpacing
};
pub use error::*;

pub fn tech_library(input: &str) -> LefReadRes<LefTechLibrary> {
    let mut builder = LefTechLibraryBuilder::default();
    
    let (input, version) = version(input)?;
    builder.version(version);
    let (input, chars) = busbit_chars(input)?;
    builder.busbitchar(chars.into());
    let (input, chars) = divider_char(input)?;
    builder.dividechar(chars.into());

    let (input, units) = units(input)?;
    builder.units(units);

    let (input, opt_mf) = opt(tuple((ws(tag("MANUFACTURINGGRID")), ws(float), ws(tag(";")))))(input)?;
    if let Some((_, manufacturing_grid, _)) = opt_mf {
        builder.manufacturing_grid(manufacturing_grid);
    }

    let (input, opt_ums) = opt(tuple((ws(tag("USEMINSPACING")), ws(identifier), ws(tag(";")))))(input)?;
    if let Some((_, use_min_spacing, _)) = opt_ums {
        match use_min_spacing {
            "ON" => builder.use_min_spacing(LefUseMinSpacing::On),
            "OFF" => builder.use_min_spacing(LefUseMinSpacing::On),
            other => {
                return Err(Err::Failure(VerboseError {
                    errors: [(other, VerboseErrorKind::Context("expected USEMINSPACING ON or OFF"))].into(),
                }));
            }
        };
    }

    let (input, layers) = many0(ws(layer))(input)?;
    builder.layers(layers);

    Ok((input, builder.build().unwrap()))
}   

fn layer(input: &str) -> LefReadRes<LefLayer> {
    let (input, _) = ws(tag("LAYER"))(input)?;
    let (input, layer_name) = ws(identifier)(input)?;
    let (input, _) = ws(tag("TYPE"))(input)?;
    let (input, layer_type) = ws(identifier)(input)?;
    let (input, _) = ws(tag(";"))(input)?;

    match layer_type {
        "CUT" => cut_layer(input, layer_name.into()).map(|(input, layer)| {
            (input, LefLayer::Cut(layer))
        }),
        "IMPLANT" => implant_layer(input, layer_name.into()).map(|(input, layer)| {
            (input, LefLayer::Implant(layer))
        }),
        "ROUTING" => routing_layer(input, layer_name.into()).map(|(input, layer)| {
            (input, LefLayer::Routing(layer))
        }),
        "MASTERSLICE" => special_layer(input, layer_name.into(), LefSpecialLayerType::MasterSlice).map(|(input, layer)| {
            (input, LefLayer::Special(layer))
        }),
        "OVERLAP" => special_layer(input, layer_name.into(), LefSpecialLayerType::Overlap).map(|(input, layer)| {
            (input, LefLayer::Special(layer))
        }),
        other => {
            return Err(Err::Failure(VerboseError {
                errors: [(other, VerboseErrorKind::Context("expected layer type"))].into(),
            }));
        }
    }
}

/*
    LAYER layerName
        TYPE CUT ;
        [MASK maskNum ;]
        [SPACING cutSpacing
            [CENTERTOCENTER]
            [SAMENET]
            [ LAYER secondLayerName [STACK]
            | ADJACENTCUTS {2 | 3 | 4} WITHIN cutWithin [EXCEPTSAMEPGNET]
            |  PARALLELOVERLAP
            | AREA cutArea
            ]
        ;] ...
        [WIDTH minWidth ;]
        [ENCLOSURE [ABOVE | BELOW] overhang1 overhang2
            [ WIDTH minWidth [EXCEPTEXTRACUT cutWithin]
            | LENGTH minLength]
        ;] ...
    END layerName
*/
fn cut_layer(input: &str, name: String) -> LefReadRes<LefCutLayer> {
    let mut builder = LefCutLayerBuilder::default();

    // [MASK maskNum ;]
    let (input, mask_opt) = opt(tuple((
        ws(tag("MASK")),
        ws(unsigned_int),
        ws(tag(";")),
    )))(input)?;
    if let Some((_, mask, _)) = mask_opt {
        builder.mask(mask);
    }

    // [SPACING cutSpacing ...
    let (input, spaces) = many0(ws(cut_layer_spacing))(input)?;
    builder.spacing(spaces);

    // [WIDTH minWidth ;]
    let (input, opt_width) = opt(tuple((
        ws(tag("WIDTH")),
        ws(float),
        ws(tag(";")),
    )))(input)?;
    if let Some((_, width, _)) = opt_width {
        builder.width(width);
    }

    // ENCLOSURE
    let (input, encloses) = many0(ws(cut_layer_enclosure))(input)?;
    builder.enclosures(encloses);
    
    // End
    let (input, _) = ws(tag("END"))(input)?;
    let (input, end_name) = ws(identifier)(input)?;
    
    if name == end_name {
        builder.name(name);
        Ok((input, builder.build().unwrap()))
    } else {
        Err(Err::Failure(VerboseError {
            errors: [(end_name, VerboseErrorKind::Context("un match end name"))].into(),
        }))
    }
}

/*
    [SPACING cutSpacing
        [CENTERTOCENTER]
        [SAMENET]
        [ LAYER secondLayerName [STACK]
        | ADJACENTCUTS {2 | 3 | 4} WITHIN cutWithin [EXCEPTSAMEPGNET]
        |  PARALLELOVERLAP
        | AREA cutArea
        ]
    ;] ...
*/
fn cut_layer_spacing(input: &str) -> LefReadRes<LefCutSpacing> {
    let (input, _) = ws(tag("SPACING"))(input)?;
    let (input, spacing) = ws(float)(input)?;

    let (input, center_to_center) = opt(ws(tag("CENTERTOCENTER")))(input)?;
    let (input, same_net) = opt(ws(tag("SAMENET")))(input)?;

    
    let (input, constraint) = opt(alt((
        tuple((ws(tag("LAYER")), ws(identifier), opt(ws(tag("STACK")))))
        .map(|(_, name, stack)| {
            LefCutSpacingConstraint::Layer {
                name: name.to_string(),
                stack: stack.is_some(),
            }
        }),

        tuple((ws(tag("ADJACENTCUTS")), ws(unsigned_int), ws(tag("WITHIN")), ws(float), opt(ws(tag("EXCEPTSAMEPGNET")))))
        .map(|(_, count, _, within, except)| {
            LefCutSpacingConstraint::AdjacentCuts {
                count: count as u8,
                within,
                except_same_pg_net: except.is_some(),
            }
        }),

        ws(tag("PARALLELOVERLAP")).map( |_| {
            LefCutSpacingConstraint::ParallelOverlap
        }),

        tuple((ws(tag("AREA")), ws(float)))
        .map(|(_, area)| {
            LefCutSpacingConstraint::Area(area)
        }),
    )))(input)?;

    let (input, _) = ws(tag(";"))(input)?;

    Ok((
        input,
        LefCutSpacing {
            cut_spacing: spacing,
            center_to_center: center_to_center.is_some(),
            same_net: same_net.is_some(),
            constraint,
        },
    ))
}

/*
    [ENCLOSURE [ABOVE | BELOW] overhang1 overhang2
        [ WIDTH minWidth [EXCEPTEXTRACUT cutWithin]
        | LENGTH minLength]
    ;] ...
*/
fn cut_layer_enclosure(input: &str) -> LefReadRes<LefEnclosure> {
    let (input, _) = ws(tag("ENCLOSURE"))(input)?;

    let (input, above_below) = opt(alt((tag("ABOVE"), tag("BELOW"))))(input)?;
    let above = match above_below {
        Some("ABOVE") => true,
        Some("BELOW") => false,
        _ => true,
    };

    let (input, overhang1) = ws(float)(input)?;
    let (input, overhang2) = ws(float)(input)?;

    let (input, condition) = opt(alt((
        tuple((
            ws(tag("WIDTH")),
            ws(float),
            opt(tuple((
                ws(tag("EXCEPTEXTRACUT")),
                ws(float),
            ))),
        ))
        .map(|(_, min_width, except)| {
            LefEnclosureCondition::Width {
                min_width,
                except_extra_cut: except.map(|(_, v)| v),
            }
        }),

        tuple((ws(tag("LENGTH")), ws(float)))
            .map(|(_, len)| LefEnclosureCondition::Length(len)),
    )))(input)?;

    let (input, _) = ws(tag(";"))(input)?;

    Ok((
        input,
        LefEnclosure {
            above,
            overhang1,
            overhang2,
            condition,
        },
    ))
}


/*
    LAYER layerName
        TYPE IMPLANT ;
        [MASK maskNum ;]
        [WIDTH minWidth ;]
        [SPACING minSpacing [LAYER layerName2] ;] ...
        [PROPERTY propName propVal ;] ...
    END layerName
*/
fn implant_layer(input: &str, name: String) -> LefReadRes<LefImplantLayer> {
    let mut builder = LefImplantLayerBuilder::default();

    // [MASK maskNum ;]
    let (input, mask_opt) = opt(tuple((
        ws(tag("MASK")),
        ws(unsigned_int),
        ws(tag(";")),
    )))(input)?;
    if let Some((_, mask, _)) = mask_opt {
        builder.mask(mask);
    }

    // [WIDTH minWidth ;]
    let (input, opt_width) = opt(tuple((
        ws(tag("WIDTH")),
        ws(float),
        ws(tag(";")),
    )))(input)?;
    if let Some((_, width, _)) = opt_width {
        builder.width(width);
    }

    // [SPACING minSpacing 
    let (input, spacings) = many0(ws(implant_layer_spacing))(input)?;
    builder.spacings(spacings);

    // [PROPERTY propName propVal ;] ...
    let (input, properties) = many0(ws(implant_layer_property))(input)?;
    builder.properties(properties);

    // End
    let (input, _) = ws(tag("END"))(input)?;
    let (input, end_name) = ws(identifier)(input)?;
    
    if name == end_name {
        builder.name(name);
        Ok((input, builder.build().unwrap()))
    } else {
        Err(Err::Failure(VerboseError {
            errors: [(end_name, VerboseErrorKind::Context("un match end name"))].into(),
        }))
    }
}

/*
    [PROPERTY propName propVal ;]
*/
fn implant_layer_property(input: &str) -> LefReadRes<(String, String)> {
    let (input, _) = ws(tag("SPACING"))(input)?;
    let (input, prop_name) = ws(identifier)(input)?;
    let (input, prop_value) = ws(identifier)(input)?;
    let (input, _) = ws(tag(";"))(input)?;

    Ok((input, (prop_name.to_string(), prop_value.to_string())))
}   

/*
    [SPACING minSpacing [LAYER layerName2] ;]
*/
fn implant_layer_spacing(input: &str) -> LefReadRes<LefImplantSpacing> {
    let mut builder = LefImplantSpacingBuilder::default();

    let (input, _) = ws(tag("SPACING"))(input)?;
    let (input, spacing) = ws(float)(input)?;
    builder.min_spacing(spacing);

    let (input, opt_layer) = opt(tuple((
        ws(tag("LAYER")),
        ws(identifier)
    )))(input)?;
    if let Some((_, layer_name)) = opt_layer {
        builder.layer(layer_name.into());        
    };
    let (input, _) = ws(tag(";"))(input)?;

    Ok((input, builder.build().unwrap()))
}

/*
    LAYER layerName
        TYPE ROUTING ;
        [MASK maskNum ;]
        DIRECTION {HORIZONTAL | VERTICAL | DIAG45 | DIAG135} ;
        PITCH {distance | xDistance yDistance} ;
        WIDTH defaultWidth ;
        [AREA minArea ;]
        [[SPACING minSpacing
            [ RANGE minWidth maxWidth
            [ USELENGTHTHRESHOLD
            | INFLUENCE value [RANGE stubMinWidth stubMaxWidth]
            | RANGE minWidth maxWidth]
            | LENGTHTHRESHOLD maxLength [RANGE minWidth maxWidth]
            | ENDOFLINE eolWidth WITHIN eolWithin
                [PARALLELEDGE parSpace WITHIN parWithin [TWOEDGES]]
            | SAMENET [PGONLY]
            | NOTCHLENGTH minNotchLength
            | ENDOFNOTCHWIDTH endOfNotchWidth NOTCHSPACING minNotchSpacing
                NOTCHLENGTH minNotchLength
            ] 
        ;] ...
        [MAXWIDTH width ;]
        [MINWIDTH width ;]
    END layerName
*/
fn routing_layer(input: &str, name: String) -> LefReadRes<LefRoutingLayer> {
    let mut builder = LefRoutingLayerBuilder::default();

    // [MASK maskNum ;]
    let (input, mask_opt) = opt(tuple((ws(tag("MASK")), ws(unsigned_int), ws(tag(";")))))(input)?;
    if let Some((_, mask, _)) = mask_opt {
        builder.mask(mask);
    }

    // DIRECTION {HORIZONTAL | VERTICAL | DIAG45 | DIAG135} ;
    let (input, _) = ws(tag("DIRECTION"))(input)?;
    let (input, direction) = alt((
        ws(tag("HORIZONTAL")).map(|_| LefRoutingDirection::Horizontal),
        ws(tag("VERTICAL")).map(|_| LefRoutingDirection::Vertical),
        ws(tag("DIAG45")).map(|_| LefRoutingDirection::Diag45),
        ws(tag("DIAG135")).map(|_| LefRoutingDirection::Diag135),
    ))(input)?;
    let (input, _) = ws(tag(";"))(input)?;
    builder.direction(direction);

    // PITCH {distance | xDistance yDistance} ;
    let (input, _) = ws(tag("PITCH"))(input)?;
    let (input, pitch1) = ws(float)(input)?;
    let (input, pitch2_opt) = opt(ws(float))(input)?;
    let pitch = match pitch2_opt {
        Some(p2) => LefPitch::XY(pitch1, p2),
        None => LefPitch::Uniform(pitch1),
    };
    let (input, _) = ws(tag(";"))(input)?;
    builder.pitch(pitch);

    // WIDTH defaultWidth ;
    let (input, _) = ws(tag("WIDTH"))(input)?;
    let (input, width) = ws(float)(input)?;
    builder.width(width);
    let (input, _) = ws(tag(";"))(input)?;

    // [AREA minArea ;]
    let (input, area_opt) = opt(tuple((ws(tag("AREA")), ws(float), ws(tag(";")))))(input)?;
    if let Some((_, area, _)) = area_opt {
        builder.area(area);
    }

    // [[SPACING minSpacing ... ;]]
    let (input, spacings) = many0(ws(parse_spacing))(input)?;
    builder.spacing_rules(spacings);

    // [MAXWIDTH width ;]
    let (input, max_width_opt) = opt(tuple((ws(tag("MAXWIDTH")), ws(float), ws(tag(";")))))(input)?;
    if let Some((_, max_width, _)) = max_width_opt {
        builder.max_width(max_width);
    }

    // [MINWIDTH width ;]
    let (input, min_width_opt) = opt(tuple((ws(tag("MINWIDTH")), ws(float), ws(tag(";")))))(input)?;
    if let Some((_, min_width, _)) = min_width_opt {
        builder.min_width(min_width);
    }

    // End
    let (input, _) = ws(tag("END"))(input)?;
    let (input, end_name) = ws(identifier)(input)?;

    if name == end_name {
        builder.name(name);
        Ok((input, builder.build().unwrap()))
    } else {
        Err(Err::Failure(VerboseError {
            errors: [(end_name, VerboseErrorKind::Context("un match end name"))].into(),
        }))
    }
}

fn parse_spacing(input: &str) -> LefReadRes<LefRoutingSpacing> {
    let (input, _) = ws(tag("SPACING"))(input)?;
    let (input, min_spacing) = ws(float)(input)?;
    // TODO: RANGE、LENGTHTHRESHOLD、SAMENET 
    let (input, _) = ws(tag(";"))(input)?;
    Ok((input, LefRoutingSpacing { min_spacing }))
}

/*
    LAYER layerName
        TYPE {MASTERSLICE | OVERLAP} ;
        [MASK maskNum ;]
        [PROPERTY propName propVal ;] ...
        [PROPERTY LEF58_TYPE
        "TYPE [NWELL | PWELL | ABOVEDIEEDGE | BELOWDIEEDGE | DIFFUSION | TRIMPOLY          | TRIMMETAL | REGION]
        ];" ;
        [PROPERTY LEF58_TRIMMEDMETAL
        "TRIMMEDMETAL metalLayer [MASK maskNum]
        ]; " ;
    END layerName
*/
fn special_layer(input: &str, name: String, tp: LefSpecialLayerType) -> LefReadRes<LefSpecialLayer> {
    let mut builder = LefSpecialLayerBuilder::default();
    builder.layer_type(tp);

    // [MASK maskNum ;]
    let (input, mask_opt) = opt(tuple((ws(tag("MASK")), ws(unsigned_int), ws(tag(";")))))(input)?;
    if let Some((_, mask, _)) = mask_opt {
        builder.mask(mask);
    }

    // [PROPERTY propName propVal ;] ...
    let (input, props) = many0(tuple((
        ws(tag("PROPERTY")),
        ws(identifier),
        ws(qstring),
        ws(tag(";")),
    )))(input)?;
    let mut normal_props = vec![];
    for (_, key, val, _) in props.iter() {
        if *key == "LEF58_TYPE" {
            let val = val.to_ascii_uppercase();
            let ty = match &*val {
                "TYPE NWELL" => Lef58Type::NWell,
                "TYPE PWELL" => Lef58Type::PWell,
                "TYPE ABOVEDIEEDGE" => Lef58Type::AboveDieEdge,
                "TYPE BELOWDIEEDGE" => Lef58Type::BelowDieEdge,
                "TYPE DIFFUSION" => Lef58Type::Diffusion,
                "TYPE TRIMPOLY" => Lef58Type::TrimPoly,
                "TYPE TRIMMETAL" => Lef58Type::TrimMetal,
                "TYPE REGION" => Lef58Type::Region,
                _ => continue,
            };
            builder.lef58_type(ty);
        } else if *key == "LEF58_TRIMMEDMETAL" {
            // value: "TRIMMEDMETAL metalLayer [MASK maskNum]"
            let (_, trimmed) = special_layer_trimmedmetal_value(val).unwrap();
            builder.lef58_trimmed_metal(trimmed);
        } else {
            normal_props.push((key.to_string(), val.to_string()));
        }
    }
    builder.properties(normal_props);

    // End
    let (input, _) = ws(tag("END"))(input)?;
    let (input, end_name) = ws(identifier)(input)?;
    
    if name == end_name {
        builder.name(name);
        Ok((input, builder.build().unwrap()))
    } else {
        Err(Err::Failure(VerboseError {
            errors: [(end_name, VerboseErrorKind::Context("un match end name"))].into(),
        }))
    }
}

fn special_layer_trimmedmetal_value(input: &str) -> LefReadRes<Lef58TrimmedMetal> {
    let (input, _) = ws(tag("TRIMMEDMETAL"))(input)?;
    let (input, metal_layer) = ws(identifier)(input)?;
    let (input, mask_opt) = opt(tuple((ws(tag("MASK")), ws(unsigned_int))))(input)?;
    let mask = mask_opt.map(|(_, v)| v);
    Ok((input, Lef58TrimmedMetal { metal_layer: metal_layer.into(), mask }))
}

/// [UNITS
///    [TIME NANOSECONDS convertFactor ;]
///    [CAPACITANCE PICOFARADS convertFactor ;]
///    [RESISTANCE OHMS convertFactor ;]
///    [POWER MILLIWATTS convertFactor ;]
///    [CURRENT MILLIAMPS convertFactor ;]
///    [VOLTAGE VOLTS convertFactor ;]
///    [DATABASE MICRONS LEFconvertFactor ;]
///    [FREQUENCY MEGAHERTZ convertFactor ;]
/// END UNITS]
fn units(input: &str) -> LefReadRes<LefUnits> {
    let mut units = LefUnits::default();
    let (input, _) = ws(tag("UNITS"))(input)?;

    // let (input, _) = many0(|input| {
    //     alt((
    //         map_res(tuple((tag("TIME"), tag("NANOSECONDS"), float, tag(";"))),
    //             |(_, _, val, _)| { units.time = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("CAPACITANCE"), tag("PICOFARADS"), float, tag(";"))),
    //             |(_, _, val, _)| { units.capacitance = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("RESISTANCE"), tag("OHMS"), float, tag(";"))),
    //             |(_, _, val, _)| { units.resistance = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("POWER"), tag("MILLIWATTS"), float, tag(";"))),
    //             |(_, _, val, _)| { units.power = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("CURRENT"), tag("MILLIAMPS"), float, tag(";"))),
    //             |(_, _, val, _)| { units.current = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("VOLTAGE"), tag("VOLTS"), float, tag(";"))),
    //             |(_, _, val, _)| { units.voltage = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("DATABASE"), tag("MICRONS"), unsigned_int, tag(";"))),
    //             |(_, _, val, _)| { units.database_microns = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //         map_res(tuple((tag("FREQUENCY"), tag("MEGAHERTZ"), float, tag(";"))),
    //             |(_, _, val, _)| { units.frequency = Some(val); Result::<(), ()>::Ok(()) }
    //         ),
    //     ))(input)
    // })(input)?;
    let (input, _) = ws(tag("DATABASE"))(input)?;
    let (input, _) = ws(tag("MICRONS"))(input)?;
    let (input, v) = ws(unsigned_int)(input)?;
    units.database_microns = Some(v);
    let (input, _) = ws(tag(";"))(input)?;

    let (input, _) = ws(tag("END"))(input)?;
    let (input, _) = ws(tag("UNITS"))(input)?;

    Ok((input, units))
}

/// [BUSBITCHARS "delimiterPair" ;]
/// 
/// Specifies the pair of characters used to specify bus bits when LEF names are mapped to or from other databases. 
/// The characters must be enclosed in double quotation marks. For example:
/// 
/// BUSBITCHARS "[]" ;
/// 
/// If one of the bus bit characters appears in a LEF name as a regular character, you must use a backslash (\) before the character to prevent the LEF reader from interpreting the character as a bus bit delimiter.
/// If you do not specify the BUSBITCHARS statement in your LEF file, the default value is "[]".
fn busbit_chars(input: &str) -> LefReadRes<&str> {
    delimited(
        ws(tag("BUSBITCHARS")),
        alt((
            ws(tag("\"[]\"")), ws(tag("\"{}\"")), ws(tag("\"<>\""))
        )),
        ws(tag(";")),
    )(input)    
}

/// [DIVIDERCHAR "character" ;]
/// Specifies the character used to express hierarchy when LEF names are mapped to or from other databases.
/// The character must be enclosed in double quotation marks. For example:
/// 
/// DIVIDERCHAR "/" ;
/// 
/// If the divider character appears in a LEF name as a regular character, you must use a backslash (\) before the character to prevent the LEF reader from interpreting the character as a hierarchy delimiter.
/// If you do not specify the DIVIDERCHAR statement in your LEF file, the default value is "/".
fn divider_char(input: &str) -> LefReadRes<&str> {
    delimited(
        ws(tag("DIVIDERCHAR")),
        alt((
            ws(tag("\"/\"")),
            ws(tag("\"\\\"")),
            ws(tag("\"%\"")),
            ws(tag("\"$\"")),
        )),
        ws(tag(";")),
    )(input)
}

/// VERSION number ;
/// Specifies which version of the LEF syntax is being used. number is a string of the form
fn version(input: &str) -> LefReadRes<f64> {
    delimited(
        ws(tag("VERSION")),
        float,
        ws(tag(";"))
    )
    (input)
}