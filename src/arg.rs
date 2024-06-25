use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// show list of available fonts
    List,
}

#[derive(Parser, Debug)]
#[command(version)]
#[command(about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// A name of font famliy
    #[arg(short, long)]
    pub famliy: Option<String>,

    /// A path to a font file
    #[arg(long)]
    pub font_file: Option<String>,

    /// A message to render, if not specified, read from stdin
    #[arg(short, long)]
    pub message: Option<String>,

    /// The height of the rendered text
    #[arg(short, long, default_value = "18")]
    pub size: u32,

    /// Each character will be offseted by this value
    ///
    /// Ex)
    /// -1.0 means the character will be moved 100% of its width to the left,
    ///
    /// 0.0 means no offset,
    ///
    /// 1.0 means the character will be moved 100% of its width to the right.
    ///
    /// Giving values like -0.5 will make the characters overlap a little. Have Fun!
    #[arg(short, long, default_value = "0.0")]
    pub offset: f32,

    /// The threshold to determine whether a pixel is foreground or background
    #[arg(short, long, default_value = "0.3")]
    pub threshold: f32,

    /// The background character
    #[arg(short, long, default_value = " ")]
    pub background: char,

    /// The mode to render the text
    ///
    /// "round": the rendered text will be rounded
    ///
    /// "square": the rendered text will be squared
    #[arg(long, default_value = "round")]
    pub mode: String,

    /// The aspect ratio of the cursor
    ///
    /// The ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
    ///
    /// Since we use (mono) character as pixel, the proper aspect ratio should be given to make the image looks good
    #[arg(short, long, default_value = "2.0")]
    pub aspect_ratio: f32,

    /// The maximum width of the rendered image
    ///
    /// If a single character cannot be fit into this width, the program will panic
    ///
    /// If the given string exceeds this width, it will be wrapped into multiple lines
    #[arg(short, long, default_value = "80")]
    pub width: usize,
}
