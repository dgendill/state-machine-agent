
use std::str::FromStr;

use dimension_parser::unit::*;
use dimension_parser::resize::*;

fn check_parse_resize_details(text: &str, result: ResizeScenario) {            
    let s = parse_resize_details(&text).into_iter().map(|x| x.scenario()).collect::<Vec<_>>();        
    assert_eq!(s, vec![result])
}

#[test]
fn test_input_formats() {

    check_parse_resize_details(
        "3000x3000",
        ResizeScenario::PixelResize(DimensionsI32 { w: 3000, h: 3000, unit: Unit::Pixels })
    );

    check_parse_resize_details(
        "5x5 in",
        ResizeScenario::PhysicalResize(DimensionsF32 { w: 5.0, h: 5.0, unit: Unit::Inches })
    );

    check_parse_resize_details(
        "3 x 2.67 in",
        ResizeScenario::PhysicalResize(DimensionsF32 { w: 3.0, h: 2.67, unit: Unit::Inches })
    );

    let units = ["inches", "cm", "in"];
    let bys = ["x", "by"];

    for by in bys {
        for unit in units {
            check_parse_resize_details(
                &format!("3000{unit}{by}3000{unit}"),
                ResizeScenario::PhysicalResize(DimensionsF32 { w: 3000.0, h: 3000.0, unit: Unit::from_str(unit).unwrap() })
            );
            check_parse_resize_details(
                &format!("3000{unit} {by} 3000{unit}"),
                ResizeScenario::PhysicalResize(DimensionsF32 { w: 3000.0, h: 3000.0, unit: Unit::from_str(unit).unwrap() })
            );
            check_parse_resize_details(
                &format!("3000 {unit} {by} 3000 {unit}"),
                ResizeScenario::PhysicalResize(DimensionsF32 { w: 3000.0, h: 3000.0, unit: Unit::from_str(unit).unwrap() })
            );
        }
    }

    let units = ["inches", "cm", "in"];
    let bys = ["x", "by"];

    for by in bys {
        for unit in units {
            check_parse_resize_details(
                &format!("3000.2{unit}{by}3000.9{unit}"),
                ResizeScenario::PhysicalResize(DimensionsF32 { w: 3000.2, h: 3000.9, unit: Unit::from_str(unit).unwrap() })
            );
            check_parse_resize_details(
                &format!("3000.2{unit} {by} 3000.9{unit}"),
                ResizeScenario::PhysicalResize(DimensionsF32 { w: 3000.2, h: 3000.9, unit: Unit::from_str(unit).unwrap() })
            );
            check_parse_resize_details(
                &format!("3000.2 {unit} {by} 3000.9 {unit}"),
                ResizeScenario::PhysicalResize(DimensionsF32 { w: 3000.2, h: 3000.9, unit: Unit::from_str(unit).unwrap() })
            );
        }
    }

}
