use std::{str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
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
        if value == "inches" || value == "inch" || value == "in" { return Ok(Unit::Inches) }
        if value == "centimeter" || value == "centimeters" || value == "cm" { return Ok(Unit::Centimeters) }
        if value == "feet" || value == "ft" { return Ok(Unit::Feet) }
        if value == "millimeter" || value == "millimeters" || value == "mm" { return Ok(Unit::Millimeters) }
        if value == "meter" || value == "meters" || value == "m" { return Ok(Unit::Meters) }
        if value == "pixel" || value == "pixels" || value == "px" { return Ok(Unit::Pixels) }        
        Err(format!("Could not convert \"{}\" into a Unit enum", value))

    }
}

impl FromStr for Unit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}