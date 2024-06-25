use font_loader::system_fonts;
use rusttype::Font;

pub mod arg;
pub mod marching;
pub mod render;

use clap::Parser;

fn main() {
    let args = arg::Args::parse();

    // check subcommands
    if let Some(arg::Command::List) = args.command {
        let families = system_fonts::query_all();
        for family in families {
            println!("{}", family);
        }
        return;
    }

    // Command line args error check
    if args.offset < -1.0 {
        panic!("--offset must be greater than or equal to -1.0");
    }

    if args.threshold < 0.0 || args.threshold > 1.0 {
        panic!("--threshold must be in the range [0.0, 1.0]");
    }

    if args.famliy.is_none() && args.font_file.is_none() {
        panic!("Either --family or --font-file must be specified");
    }

    // load font data
    let font_data = if let Some(family) = args.famliy {
        // Get the system font
        let property = system_fonts::FontPropertyBuilder::new()
            .family(&family)
            .build();

        let (font_data, _) = system_fonts::get(&property)
            .expect(format!("Failed to find system font: {}", &family).as_str());
        font_data
    } else {
        // read from file
        let font_file_path = args.font_file.unwrap();
        std::fs::read(&font_file_path)
            .expect(format!("Failed to read font file: {}", &font_file_path).as_str())
    };
    let font = Font::try_from_bytes(&font_data).expect("Failed to load font");

    // render
    let size = (args.size - 1) as usize;

    // if --message is not specified, read from stdin
    let message = if let Some(message) = args.message {
        message
    } else {
        let mut message = String::new();
        std::io::stdin()
            .read_line(&mut message)
            .expect("Failed to read from stdin");
        message
    };

    let image = render::render_str(
        &font,
        &message.trim(),
        size,
        args.offset,
        args.threshold,
        args.aspect_ratio,
        Some(args.width),
    );
    if let Err(ch) = image {
        panic!("Character '{}' cannot be fit into max_width", ch);
    }
    let image = image.unwrap();

    let ms: Box<dyn marching::MarchingSquare> = match args.mode.as_str() {
        "round" | "\"round\"" => Box::new(marching::RoundCornerMarchingSquare {
            null: args.background,
            full: ' ',
        }),
        "square" | "\"square\"" => Box::new(marching::SharpCornerMarchingSquare {
            null: args.background,
            full: ' ',
        }),
        _ => panic!("Invalid --mode"),
    };

    for big_line in image.into_iter() {
        let lines = marching::marching_square(&big_line, size, Some(false), ms.as_ref());
        for line in lines {
            println!("{}", line);
        }
    }
}
