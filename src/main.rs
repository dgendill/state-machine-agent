use rustyline::{DefaultEditor, error::ReadlineError};
use dimension_parser::resize::parse_resize_details;

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
    let _ = rl.save_history("history.txt");
    Ok(())

}