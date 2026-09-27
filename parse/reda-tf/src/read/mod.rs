mod ast;
mod error;
mod technology;
mod color;
mod tile;
mod layer;
mod contact;
mod design_rule;
use ast::{Rule, TfPestParser};
use color::parse_color_define;
use contact::parse_contact_code;
use design_rule::parse_design_rule;
pub use error::*;
use layer::parse_layer;
use pest::Parser;
use technology::parse_technology;
use tile::parse_tile;
use crate::{Techfile, TechfileBuilder};

pub fn parse_techfile(input: &str) -> TfReadResult<Techfile> {
    let mut pairs = TfPestParser::parse(Rule::file, input)?;
    let file_pair = pairs.next().unwrap();

    let mut builder = TechfileBuilder::default();
    let mut colors = vec![];
    let mut tiles = vec![];
    let mut layers = vec![];
    let mut contacts = vec![];
    let mut design_rules = vec![];

    for pair in file_pair.into_inner() {
        match pair.as_rule() {
            Rule::technology => { builder.technology(parse_technology(pair)?); }
            Rule::color => { colors.push(parse_color_define(pair)?); }
            Rule::tile => { tiles.push(parse_tile(pair)?); }
            Rule::layer => { layers.push(parse_layer(pair)?); }
            Rule::contact_code => { contacts.push(parse_contact_code(pair)?); }
            Rule::design_rule => { design_rules.push(parse_design_rule(pair)?); }
            Rule::EOI => (),
            _ => unreachable!("Unexpected top level rule: {:?}", pair.as_rule()),
        }
    }

    builder.colors(colors);
    builder.tiles(tiles);
    builder.layers(layers);
    builder.contacts(contacts);
    builder.design_rules(design_rules);
    let tech_file = builder.build()?;
    Ok(tech_file)
}

#[cfg(test)]
mod test {
    use super::parse_techfile;

    #[test]
    fn test_simple() {
        let sample_input = r#"
Technology	{
    name				= ""
    dielectric			= 3.45e-05
    unitTimeName			= "ns"
    timePrecision			= 1000
    unitLengthName			= "micron"
    lengthPrecision			= 2000
    gridResolution			= 10
    unitVoltageName			= "V"
    voltagePrecision		= 1000000
    unitCurrentName			= "mA"
    currentPrecision		= 1000000
    unitPowerName			= "mw"
    powerPrecision			= 1000000
    unitResistanceName		= "kohm"
    resistancePrecision		= 1000000
    unitCapacitanceName		= "pf"
    capacitancePrecision		= 1000000
    unitInductanceName		= "nh"
    inductancePrecision		= 100
    minBaselineTemperature		= 25
    nomBaselineTemperature		= 25
    maxBaselineTemperature		= 25
}

Color		43 {
        name				= "43"
        rgbDefined			= 1
        redIntensity			= 180
        greenIntensity			= 175
        blueIntensity			= 255
}

Color		47 {
        name				= "47"
        rgbDefined			= 1
        redIntensity			= 180
        greenIntensity			= 255
        blueIntensity			= 255
}

Tile		"unit" {
        width				= 0.19
        height				= 1.4
}

Layer		"poly" {
        maskName			= ""
        layerNumber			= 1
        visible				= 1
        selectable			= 1
        blink				= 0
        color				= "white"
        lineStyle			= "solid"
        pattern				= "blank"
        pitch				= 0
        defaultWidth			= 0
        minWidth			= 0
        minSpacing			= 0
}

Layer		"metal5" {
		layerNumber			= 11
		maskName			= "metal5"
		isDefaultLayer      = 1
		visible				= 1
		selectable			= 1
		blink				= 0
		color				= "magenta"
		lineStyle			= "solid"
		pattern				= "backSlash"
		pitch				= 0.28
		defaultWidth			= 0.14
		minWidth			= 0.14
		minSpacing			= 0.14
		sameNetMinSpacing		= 0.14
		unitMinResistance		= 0.00021
		unitNomResistance		= 0.00021
		unitMaxResistance		= 0.00021
		unitMinCapacitance		= 4.8e-05
		unitNomCapacitance		= 4.8e-05
		unitMaxCapacitance		= 4.8e-05
		unitMinHeightFromSub		= 1.71
		unitNomHeightFromSub		= 1.71
		unitMaxHeightFromSub		= 1.71
		unitMinThickness		= 0.28
		unitNomThickness		= 0.28
		unitMaxThickness		= 0.28
		fatTblDimension			= 5
		fatTblThreshold			= (0,0.2705,0.5005,0.9005,1.5005)
		fatTblParallelLengthDimension	= 5
		fatTblParallelLength		= (0,0.9005,1.8005,2.7005,4.0005)
		fatTblSpacing			= (0.14,0.14,0.14,0.14,0.14,
						   0.14,0.27,0.27,0.27,0.27,
						   0.14,0.27,0.5,0.5,0.5,
						   0.14,0.27,0.5,0.9,0.9,
						   0.14,0.27,0.5,0.9,1.5)
}

ContactCode	"via2_5" {
    contactCodeNumber		= 7
    cutLayer			= "via2"
    lowerLayer			= "metal2"
    upperLayer			= "metal3"
    isDefaultContact		= 1
    cutWidth			= 0.07
    cutHeight			= 0.07
    upperLayerEncWidth		= 0.035
    upperLayerEncHeight		= 0
    lowerLayerEncWidth		= 0
    lowerLayerEncHeight		= 0.035
    minCutSpacing			= 0.09
    unitMinResistance		= 0.005
    unitNomResistance		= 0.005
    unitMaxResistance		= 0.005
}

ContactCode	"via2_6" {
    contactCodeNumber		= 8
    cutLayer			= "via2"
    lowerLayer			= "metal2"
    upperLayer			= "metal3"
    isDefaultContact		= 1
    cutWidth			= 0.07
    cutHeight			= 0.07
    upperLayerEncWidth		= 0.035
    upperLayerEncHeight		= 0.035
    lowerLayerEncWidth		= 0.035
    lowerLayerEncHeight		= 0
    minCutSpacing			= 0.09
    unitMinResistance		= 0.005
    unitNomResistance		= 0.005
    unitMaxResistance		= 0.005
}

DesignRule	{
		layer1				= "via3"
		layer2				= "via4"
		minSpacing			= 0
		minEnclosure			= 0
		stackable			= 1
		stackViaCenterSpacingThreshold	= 0
}

DesignRule	{
		layer1				= "via4"
		layer2				= "via5"
		minSpacing			= 0
		minEnclosure			= 0
		stackable			= 1
		stackViaCenterSpacingThreshold	= 0
}
        "#;
    
        let tf = parse_techfile(sample_input).unwrap();
        println!("Parsed Successfully!");
        println!("Tech Name: {:?}", tf.technology.name);
        assert_eq!(2, tf.colors.len());
        assert_eq!(2, tf.layers.len());
        assert_eq!(2, tf.contacts.len());
        assert_eq!(2, tf.design_rules.len());
    }
}
