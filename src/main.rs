use std::{fmt::Display, str::FromStr};

use regex::Regex;
use rustyline::{DefaultEditor, error::ReadlineError};

#[derive(Debug, Clone, PartialEq)]
enum Unit {
    Inches,
    Centimeters,
    Feet,
    Millimeters,
    Meters,
    Pixels
}

impl TryFrom<&str> for Unit {
    type Error = String;    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if (value == "inches" || value == "inch" || value == "in") { return Ok(Unit::Inches) }
        if (value == "centimeter" || value == "centimeters" || value == "cm") { return Ok(Unit::Centimeters) }
        if (value == "feet" || value == "ft") { return Ok(Unit::Feet) }
        if (value == "millimeter" || value == "millimeters" || value == "mm") { return Ok(Unit::Millimeters) }
        if (value == "meter" || value == "meters" || value == "m") { return Ok(Unit::Meters) }
        if (value == "pixel" || value == "pixels" || value == "px") { return Ok(Unit::Pixels) }        
        Err(format!("Could not convert \"{}\" into a Unit enum", value))

    }
}

impl FromStr for Unit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

enum Tool {
    ImageSize(fn() -> (i32, i32)),
    AspectRatio(fn() -> f32),
    DimensionToInches(fn(Unit, f32) -> f32)
}


#[derive(Debug, Clone, PartialEq)]
struct DimensionsI32 {
    w: i32,
    h: i32,
    unit: Unit
}

#[derive(Debug, Clone, PartialEq)]
struct DimensionsF32 {
    w: f32,
    h: f32,
    unit: Unit
}


#[derive(Debug, Clone, PartialEq)]
struct RawResizeDetails {    
    raw_width: String,
    raw_height: String,
    raw_inline_unit: Option<String>,
    raw_unit: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
enum ResizeScenario {
    DecimalResize(DimensionsF32),
    WholeResize(DimensionsI32),
    ClarifyUnit(RawResizeDetails),
    FractionalPixels(RawResizeDetails)
}

impl RawResizeDetails {
    fn has_decimal(&self) -> bool {
        self.raw_width.contains(".") || self.raw_height.contains(".")
    }

    fn scenario(self) -> ResizeScenario {
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
        
        if self.has_decimal() {
            ResizeScenario::DecimalResize(DimensionsF32 {
                w: self.raw_width.parse::<f32>().unwrap(),
                h: self.raw_height.parse::<f32>().unwrap(),
                unit
            })
        } else {
            ResizeScenario::WholeResize(DimensionsI32 {
                w: self.raw_width.parse::<i32>().unwrap(),
                h: self.raw_height.parse::<i32>().unwrap(),
                unit
            })
        }    
    }
}

fn parse_resize_details(text: &str) -> Vec<RawResizeDetails> {
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


fn main() -> rustyline::Result<()> {

    let mut context : Vec<String> = vec![];

    // tools
    // let tools = vec![
    //     Tool::ImageSize(get_image_size),
    //     Tool::AspectRatio(get_aspect_ratio),
    //     Tool::DimensionToInches(dimension_to_inches),
    //     Tool::ImageSize(get_image_size),
    // ];
    
    // SET THE GOAL    
    // SET THE CONTEXT    
    // TOOL(S) GET CALLED BASED ON THE CONTEXT
   

    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                let size = parse_resize_details(&line).into_iter().map(|x| x.scenario()).collect::<Vec<_>>();
                println!("{size:?}");
                context.push(line);

            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    


        // AWAIT CONTEXT CHANGE
        // loop {        
        // DETERMINE NEEDED TOOL USE
        // USE TOOL
        // OUTPUT
        // UPDATE CONTEXT
        // END LOOP IF NECESSARY
        // }
        //
        // OUTPUT
        
    }

    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())

}

#[cfg(test)]
mod test {

    use super::*;

    fn check_parse_resize_details(text: &str, result: ResizeScenario) {            
        let s = parse_resize_details(&text).into_iter().map(|x| x.scenario()).collect::<Vec<_>>();        
        assert_eq!(s, vec![result])
    }

    fn test_happy_paths() {        
        // "I want to resize the image to 500x500"        
        // "I want to resize the image to 5x5 in"
        // "I want to resize the image to 3 x 2.67 in"

    }


    #[test]
    fn test_input_formats() {

        check_parse_resize_details(
            "3000x3000",
            ResizeScenario::WholeResize(DimensionsI32 { w: 3000, h: 3000, unit: Unit::Pixels })
        );

        check_parse_resize_details(
            "5x5 in",
            ResizeScenario::WholeResize(DimensionsI32 { w: 5, h: 5, unit: Unit::Inches })
        );

        check_parse_resize_details(
            "3 x 2.67 in",
            ResizeScenario::DecimalResize(DimensionsF32 { w: 3.0, h: 2.67, unit: Unit::Inches })
        );

        let units = ["inches", "cm", "in", "pixels"];
        let bys = ["x", "by"];

        for by in bys {
            for unit in units {
                check_parse_resize_details(
                    &format!("3000{unit}{by}3000{unit}"),
                    ResizeScenario::WholeResize(DimensionsI32 { w: 3000, h: 3000, unit: Unit::from_str(unit).unwrap() })
                );
                check_parse_resize_details(
                    &format!("3000{unit} {by} 3000{unit}"),
                    ResizeScenario::WholeResize(DimensionsI32 { w: 3000, h: 3000, unit: Unit::from_str(unit).unwrap() })
                );
                check_parse_resize_details(
                    &format!("3000 {unit} {by} 3000 {unit}"),
                    ResizeScenario::WholeResize(DimensionsI32 { w: 3000, h: 3000, unit: Unit::from_str(unit).unwrap() })
                );
            }
        }

        let units = ["inches", "cm", "in"];
        let bys = ["x", "by"];

        for by in bys {
            for unit in units {
                check_parse_resize_details(
                    &format!("3000.2{unit}{by}3000.9{unit}"),
                    ResizeScenario::DecimalResize((DimensionsF32 { w: 3000.2, h: 3000.9, unit: Unit::from_str(unit).unwrap() }))
                );
                check_parse_resize_details(
                    &format!("3000.2{unit} {by} 3000.9{unit}"),
                    ResizeScenario::DecimalResize(DimensionsF32 { w: 3000.2, h: 3000.9, unit: Unit::from_str(unit).unwrap() })
                );
                check_parse_resize_details(
                    &format!("3000.2 {unit} {by} 3000.9 {unit}"),
                    ResizeScenario::DecimalResize(DimensionsF32 { w: 3000.2, h: 3000.9, unit: Unit::from_str(unit).unwrap() })
                );
            }
        }

    }
}