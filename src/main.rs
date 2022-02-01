use std::{env, str};
use std::path::PathBuf;
use image::RgbaImage;
use rand::Rng;

// debug messages
const USAGE: &'static str = "Usage: camoufler <input_image> <output_folder> [<arguments>]";

fn main() {

    /*
    **  Process arguments
    */

    // initialise argument values
    let mut input_path_raw = String::new();
    let mut output_path_raw = String::new();
    let mut smudge_weight: u8 = 1;
    let mut smudge_shade: bool = false;

    // modify misc argument values
    match read_args(&mut smudge_weight, &mut smudge_shade, &mut input_path_raw, &mut output_path_raw) {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR whilst reading arguments: {}\n{}", e, USAGE);
            return;
        }
    }

    let input_path: PathBuf;
    let output_path: PathBuf;

    // determine that the input path is a valid file
    match parse_input_path(input_path_raw) {
        Ok(path) => {
            input_path = path;
        },
        Err(e) => {
            println!("ERROR whilst reading input: {}\n{}", e, USAGE);
            return;
        }
    }

    // determine that the output path is a valid directory
    match parse_output_path(output_path_raw) {
        Ok(path) => {
            output_path = path;
        },
        Err(e) => {
            println!("ERROR whilst reading output: {}\n{}", e, USAGE);
            return;
        }
    }

    /*
    **  Process & generate image from path
    */

    let mut input_image: RgbaImage;

    // verify that a valid image has been specified
    match image::open(input_path) {
        Ok(img) => {
            input_image = img.to_rgba8();
        },
        Err(e) => {
            println!("ERROR whilst processing image: {}\n{}", e, USAGE);
            return;
        }
    }

    smudge_image(&mut input_image, &smudge_weight, &smudge_shade);
    let status = image::save_buffer(output_path.join("output.png"), &input_image, input_image.width(), input_image.height(), image::ColorType::Rgba8);
    match status {
        Ok(_) => println!("Successfuly saved output.png to {:?}", output_path),
        Err(e) => println!("ERROR whilst saving image: {}\n{}", e, USAGE)
    }
}

/*
**  ARGUMENT HANDLERS
*/

fn read_args(smudge_weight: &mut u8, smudge_shade: &mut bool, input_path: &mut String, output_path: &mut String) -> Result<bool, String> {

    // construct values from arguments
    let args: Vec<String> = env::args().collect();
    let end = args.len();
    let mut start = 1;

    // populate argument values
    while start < end {

        for i in start..end {
            start = i;
            let arg = &args[i];
            let arg_value_ref = i+1;

            // read input/output path arguments first

            if i == 1 {
                *input_path = String::from(arg);
                start += 1;
                break;
            }

            else if i == 2 {
                *output_path = String::from(arg);
                start += 1;
                break;
            }

            // miscellaneous arguments; MUST require a trailing value

            else if arg_value_ref >= end {
                let msg = String::from("All arguments must correspond with a value.");
                return Err::<bool, String>(msg);
            }
            
            else if arg.eq("--smudge-weight") || arg.eq("-w") {
                *smudge_weight = parse_arg(&args[arg_value_ref], 1);
                start += 2;
                break;
            }

            else if arg.eq("--smudge-shade") || arg.eq("-s") {
                *smudge_shade = parse_arg(&args[arg_value_ref], false);
                start += 2;
                break;
            }

            // ERROR: Argument that does not exist
            else {
                let msg = format!("Invalid argument \"{}\" specified!", &arg);
                return Err::<bool, String>(msg);
            }
        }
    }

    Ok(true)
}

fn parse_arg<T: str::FromStr>(arg: &str, default: T) -> T {
    let value = arg.parse();
    match value {
        Ok(v) => v,
        Err(_) => {
            println!("WARNING: Invalid value \"{}\" specified; using default value instead.", arg);
            default
        }
    }
}

fn parse_input_path(path: String) -> Result<PathBuf, &'static str> {
    let empty = path.is_empty();
    let value = PathBuf::from(path);

    if empty {
        return Err("No input path was specified!");
    }

    else if !value.exists() {
        return Err("Specified input file path does not exist!");
    }

    else if value.is_dir() {
        return Err("Expected file in input path, found directory instead.");
    }

    Ok(value)
}

fn parse_output_path(path: String) -> Result<PathBuf, &'static str> {
    let empty = path.is_empty();
    let value = PathBuf::from(path);

    if empty {
        return Err("No output path was specified!");
    }

    else if !value.exists() {
        return Err("Specified output directory path does not exist!");
    }

    else if !value.is_dir() {
        return Err("Output path needs to be a directory!");
    }

    Ok(value)
}

/*
**  IMAGE HANDLERS
*/

fn smudge_image(image: &mut RgbaImage, weight: &u8, shade: &bool) {
    // determine the maximum value that the image should be smudged
    let smudge: isize = if *shade {
        -1
    } else {
        1
    } * (*weight as isize);

    // iterate pixels of image to smudge its colours
    for pixel in image.pixels_mut() {
        for i in 0..2 {
            let colour = &mut pixel[i];
            let smudged = rand::thread_rng().gen_range(0..*weight) as isize;
            let new_colour = *colour as isize + (smudge * smudged);
            // within u8 range
            *colour = if new_colour <= 0 {
                0
            } else if new_colour >= 255 {
                255
            } else {
                new_colour as u8
            }
        }
    }
}