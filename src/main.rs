use std::{fs, path::{Path, PathBuf}, process};

use rustyline::{DefaultEditor, error::ReadlineError};
use dimension_parser::{resize::{ResizeScenario, parse_resize_details}, unit};

#[derive(Debug)]
struct State {
    image: Option<PathBuf>,
    native_image_width: Option<i32>,
    native_image_height: Option<i32>,
    scenario: Option<ResizeScenario>,
    is_upscaling: bool,
    dpi: Option<i32>
}

fn select_image(state: &mut State, text: &str) -> Result<String, String> {
    if PathBuf::from(text).exists() {
        let path = PathBuf::from(text);
        let _img = image::ImageReader::open(&path)
            .map_err(|e| String::from("Could not open file as an image."))?
            .with_guessed_format()
            .map_err(|e| String::from("Could not open file as an image."))?
            .decode()
            .map_err(|e| String::from("Could not open file as an image."))?;

        state.native_image_width = Some(_img.width() as i32);
        state.native_image_height = Some(_img.height() as i32);
        state.image = Some(path);
        return Ok("Ok, we can resize that image.".into());
    } else {
        return Err("File doesn't exist".into());
    }
}

fn get_dimensions(state: &mut State, text: &str) -> Result<String, String> {
    let mut details = parse_resize_details(text);
    if details.len() == 1 {
        let details = details.swap_remove(0);
        state.scenario = Some(details.scenario());
        return Ok("".into());
    } else {
        return Ok("Sorry, I'm confused what dimensions you want to use.".into());
    }    
}

fn parse_dpi(state: &mut State, text: &str) -> Result<String, String> {
    let dpi = text.parse::<i32>().map_err(|e| String::from("Could not read DPI from your input."))?;
    state.dpi = Some(dpi);
    Ok(dpi.to_string())
}


fn exit_ok(state: &mut State, text: &str) -> Result<String, String> {
    process::exit(0);
}

fn exit_cancel(state: &mut State, text: &str) -> Result<String, String> {
    process::exit(1);
}

fn reenter_dimensions(state: &mut State, text: &str) -> Result<String, String> {
    state.scenario = None;
    state.dpi = None;
    Ok("".into())
}

fn confirm_plan(state: &mut State, text: &str) -> Result<String, String> {
    match text.to_ascii_lowercase().as_str() {
        "yes" | "y" => {
            println!("Ok. Complete. (simulated)");
            exit_ok(state, text);
            Ok("".into())
        },
        _ => {            
            println!("Canceling workflow.");
            exit_cancel(state, text);
            Ok("".into())
        }
    }
}


fn noop(state: &mut State, text: &str) -> Result<String, String> {
    Ok("".into())
}

impl State {
    fn handle(&self) -> (String, fn (&mut State, text: &str) -> Result<String, String>) {
        if self.image.is_none() {
            return (
                String::from("What image would you like to resize?"),
                select_image
            );                
        } else if self.image.is_some() && self.scenario.is_none() {
            return (
                String::from("What dimensions do you want for the resized image?"),
                get_dimensions
            )
        } else if self.scenario.is_some() {

            let s = self.scenario.as_ref().unwrap();
            
            match s {
                ResizeScenario::ClarifyUnit(_) => {
                    return (
                        String::from("Sorry, I'm not exactly sure what unit you want to use."),
                        noop
                    )
                },
                ResizeScenario::PhysicalResize(dimensions) => {
                    if self.dpi.is_none() {
                        return (
                            String::from("What dpi value do you want? (300 is ideal)"),
                            parse_dpi
                        )
                    } else {
                        let dpi = self.dpi.unwrap();                                        
                        let inches_width = unit::unit_to_inches(&dimensions.unit, dimensions.w);
                        let inches_height = unit::unit_to_inches(&dimensions.unit, dimensions.h);
                        let pixels_width = f32::round(inches_width * dpi as f32) as i32;
                        let pixels_height = f32::round(inches_height * dpi as f32) as i32;

                        return (
                            format!("Ok. I can resize {image} to {width} x {height} pixels. Do you want me to do this for you?",
                                image=self.image.as_ref().unwrap().display(),
                                width=pixels_width,
                                height=pixels_height
                            ),
                            confirm_plan
                        )                
                    }                    
                },
                ResizeScenario::PixelResize(dimensions) => {
                    return (
                        format!("Ok. I can resize {image} to {width} x {height} pixels for you. Do you want me to do this for you?",
                            image=self.image.as_ref().unwrap().display(),
                            width=dimensions.w,
                            height=dimensions.h
                        ),
                        confirm_plan
                    )  
                },
                ResizeScenario::FractionalPixels(_) => {
                    return (
                        String::from("You should use whole pixel values for your dimensions or specify a different unit."),
                        reenter_dimensions
                    )
                }
                
            }
        } else {
            return (
                String::from(""),
                noop
            )
        }
    }
}

fn main() -> rustyline::Result<()> {

    let mut state = State {
        image: None,
        scenario: None,
        native_image_width: None,
        native_image_height: None,
        is_upscaling: false,
        dpi: None
    };

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
        let (text, handler) = state.handle();
        println!("{text}");
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                let result = handler(&mut state, &line);
                
                match result {
                    Ok(r) => {
                        if r != "" { println!("{r}") }
                    }
                    Err(e) => println!("{e}"),
                }
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
    let _ = rl.save_history("history.txt");
    Ok(())

}