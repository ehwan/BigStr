/// marching squares

// for round corners
const ROUND_TOP_LEFT: char = '\u{256F}';
const ROUND_TOP_RIGHT: char = '\u{2570}';
const ROUND_BOTTOM_LEFT: char = '\u{256E}';
const ROUND_BOTTOM_RIGHT: char = '\u{256D}';

// for sharp corners
const SHARP_TOP_LEFT: char = '\u{2518}';
const SHARP_TOP_RIGHT: char = '\u{2514}';
const SHARP_BOTTOM_LEFT: char = '\u{2510}';
const SHARP_BOTTOM_RIGHT: char = '\u{250C}';

const VERTICAL: char = '\u{2502}';
const HORIZONTAL: char = '\u{2500}';

const DIAG_LEFT_DOWN: char = '\u{2571}';
const DIAG_RIGHT_DOWN: char = '\u{2572}';
// const XCROSS: char = '\u{2573}';
// const CROSS: char = '\u{253C}';

pub trait MarchingSquare {
    fn marching_square(
        &self,
        top_left: bool,
        top_right: bool,
        bottom_left: bool,
        bottom_right: bool,
    ) -> char;
}

pub struct RoundCornerMarchingSquare {
    pub null: char,
    pub full: char,
}

impl MarchingSquare for RoundCornerMarchingSquare {
    fn marching_square(
        &self,
        top_left: bool,
        top_right: bool,
        bottom_left: bool,
        bottom_right: bool,
    ) -> char {
        let index = (top_left as u8)
            | (top_right as u8) << 1
            | (bottom_left as u8) << 2
            | (bottom_right as u8) << 3;

        // each bit represents a vertex of the cube
        /*
         0----1
         |    |
         2----3
        */
        match index {
            0b0000 => self.null,
            0b0001 => ROUND_TOP_LEFT,
            0b0010 => ROUND_TOP_RIGHT,
            0b0011 => HORIZONTAL,
            0b0100 => ROUND_BOTTOM_LEFT,
            0b0101 => VERTICAL,
            0b0110 => DIAG_LEFT_DOWN,
            0b0111 => ROUND_BOTTOM_RIGHT,
            0b1000 => ROUND_BOTTOM_RIGHT,
            0b1001 => DIAG_RIGHT_DOWN,
            0b1010 => VERTICAL,
            0b1011 => ROUND_BOTTOM_LEFT,
            0b1100 => HORIZONTAL,
            0b1101 => ROUND_TOP_RIGHT,
            0b1110 => ROUND_TOP_LEFT,
            0b1111 => self.full,
            _ => panic!("Invalid index"),
        }
    }
}

pub struct SharpCornerMarchingSquare {
    pub null: char,
    pub full: char,
}

impl MarchingSquare for SharpCornerMarchingSquare {
    fn marching_square(
        &self,
        top_left: bool,
        top_right: bool,
        bottom_left: bool,
        bottom_right: bool,
    ) -> char {
        let index = (top_left as u8)
            | (top_right as u8) << 1
            | (bottom_left as u8) << 2
            | (bottom_right as u8) << 3;

        // each bit represents a vertex of the cube
        /*
         0----1
         |    |
         2----3
        */
        match index {
            0b0000 => self.null,
            0b0001 => SHARP_TOP_LEFT,
            0b0010 => SHARP_TOP_RIGHT,
            0b0011 => HORIZONTAL,
            0b0100 => SHARP_BOTTOM_LEFT,
            0b0101 => VERTICAL,
            0b0110 => DIAG_LEFT_DOWN,
            0b0111 => SHARP_BOTTOM_RIGHT,
            0b1000 => SHARP_BOTTOM_RIGHT,
            0b1001 => DIAG_RIGHT_DOWN,
            0b1010 => VERTICAL,
            0b1011 => SHARP_BOTTOM_LEFT,
            0b1100 => HORIZONTAL,
            0b1101 => SHARP_TOP_RIGHT,
            0b1110 => SHARP_TOP_LEFT,
            0b1111 => self.full,
            _ => panic!("Invalid index"),
        }
    }
}

/// performs marching square algorithm to given image
/// and returns the result as a vector of strings
/// each string represents a row of the image
/// if `pad` is given, it will be added to each side of the image
/// the size of returned vector will be (w + 2*pad - 1) * (h + 2*pad - 1)
pub fn marching_square(
    image: &[bool],          // image buffer, should be row-major, top-left origin
    h: usize,                // height of image
    pad: Option<bool>,       // should add padding to the image?
    ms: &dyn MarchingSquare, // MarchingSquare implementation
) -> Vec<String> {
    let w = image.len() / h;
    let mut ret = Vec::new();

    if let Some(pad) = pad {
        ret.reserve(h + 1);

        // y = -1
        {
            let mut row = String::new();
            row.reserve((w + 1) * 2 + 1);

            // x = -1
            row.push(ms.marching_square(pad, pad, pad, image[0]));
            // x = 0..w-1
            for x in 0..w - 1 {
                row.push(ms.marching_square(pad, pad, image[x], image[x + 1]));
            }
            // x = w-1
            row.push(ms.marching_square(pad, pad, image[w - 1], pad));

            ret.push(row);
        }

        // y = 0..h-1
        for y in 0..h - 1 {
            let mut row = String::new();
            row.reserve((w + 1) * 2 + 1);

            row.push(ms.marching_square(pad, image[y * w + 0], pad, image[(y + 1) * w + 0]));
            for x in 0..w - 1 {
                row.push(ms.marching_square(
                    image[y * w + x],
                    image[y * w + x + 1],
                    image[(y + 1) * w + x],
                    image[(y + 1) * w + x + 1],
                ));
            }
            row.push(ms.marching_square(
                image[y * w + w - 1],
                pad,
                image[(y + 1) * w + w - 1],
                pad,
            ));

            ret.push(row);
        }

        // y = h-1
        {
            let mut row = String::new();
            row.reserve((w + 1) * 2 + 1);

            // x = -1
            row.push(ms.marching_square(pad, image[(h - 1) * w], pad, pad));
            // x = 0..w-1
            for x in 0..w - 1 {
                row.push(ms.marching_square(
                    image[(h - 1) * w + x],
                    image[(h - 1) * w + x + 1],
                    pad,
                    pad,
                ));
            }
            // x = w-1
            row.push(ms.marching_square(image[(h - 1) * w + w - 1], pad, pad, pad));

            ret.push(row);
        }
    } else {
        ret.reserve(h - 1);

        for y in 0..h - 1 {
            let mut row = String::new();
            row.reserve((w - 1) * 2 + 1);
            for x in 0..w - 1 {
                row.push(ms.marching_square(
                    image[y * w + x],
                    image[y * w + x + 1],
                    image[(y + 1) * w + x],
                    image[(y + 1) * w + x + 1],
                ));
            }

            ret.push(row);
        }
    }

    ret
}
