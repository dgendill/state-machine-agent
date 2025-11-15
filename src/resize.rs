use regex::Regex;

use super::unit::{Unit};

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionsI32 {
    pub w: i32,
    pub h: i32,
    pub unit: Unit
}

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionsF32 {
    pub w: f32,
    pub h: f32,
    pub unit: Unit
}


#[derive(Debug, Clone, PartialEq)]
pub struct RawResizeDetails {    
    pub raw_width: String,
    pub raw_height: String,
    pub raw_inline_unit: Option<String>,
    pub raw_unit: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResizeScenario {
    PhysicalResize(DimensionsF32),
    PixelResize(DimensionsI32),
    ClarifyUnit(RawResizeDetails),
    FractionalPixels(RawResizeDetails)
}

impl RawResizeDetails {
    pub fn has_decimal(&self) -> bool {
        self.raw_width.contains(".") || self.raw_height.contains(".")
    }

    pub fn scenario(self) -> ResizeScenario {
        if self.raw_inline_unit.is_some() && self.raw_unit.is_some() && self.raw_inline_unit != self.raw_unit {
            return ResizeScenario::ClarifyUnit(self)
        }

        let unit = if self.raw_inline_unit.is_none() && self.raw_unit.is_none() {
            Unit::Pixels
        } else {
            self.raw_inline_unit.clone().or(self.raw_unit.clone()).clone().unwrap().parse::<Unit>().unwrap()
        };

        if self.has_decimal() && unit == Unit::Pixels {
            return ResizeScenario::FractionalPixels(self)
        }
        
        if unit == Unit::Pixels {
            ResizeScenario::PixelResize(DimensionsI32 {
                w: self.raw_width.parse::<i32>().unwrap(),
                h: self.raw_height.parse::<i32>().unwrap(),
                unit
            })
        } else {
            ResizeScenario::PhysicalResize(DimensionsF32 {
                w: self.raw_width.parse::<f32>().unwrap(),
                h: self.raw_height.parse::<f32>().unwrap(),
                unit
            })
        }    
    }
}

pub fn parse_resize_details(text: &str) -> Vec<RawResizeDetails> {
    let re = Regex::new(r"(?<width>[0-9]+([.][0-9]+)?)[ ]*(?<inline_unit>(pixels|pixel|px|inches|inch|in|centimeters|centimeter|cm|millimeters|millimeter|mm|meters|meter|m|ft|feet))?[ ]*(x|by)[ ]*(?<height>[0-9]+([.][0-9]+)?)(<inline_unit>)?[ ]*(?<unit>(pixels|pixel|px|inches|inch|in|centimeters|centimeter|cm|millimeters|millimeter|mm|meters|meter|ft|feet))?").unwrap();
    let mut results = vec![];

    for result in re.captures_iter(text) {        
        let width = result.name("width");
        let height = result.name("height");
        if width.is_none() || height.is_none() { continue; }

        let inline_unit = result.name("inline_unit").map(|x| x.as_str().to_string());
        let unit = result.name("unit").map(|x| x.as_str().to_string());
    
        results.push(RawResizeDetails {
            raw_width: width.unwrap().as_str().to_string(),
            raw_height: height.unwrap().as_str().to_string(),
            raw_inline_unit: inline_unit,
            raw_unit: unit
        });        
    }

    results
}