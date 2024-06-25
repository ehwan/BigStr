use rusttype::{Font, Scale};

pub fn render_char<'a>(font: &Font<'a>, ch: char, line_height: usize) -> Vec<f32> {
    // the ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
    let cursor_aspect_ratio = 2.0;
    let scale = Scale {
        x: line_height as f32 * cursor_aspect_ratio,
        y: line_height as f32,
    };
    let vmetrics = font.v_metrics(scale);
    let offset = rusttype::point(0.0, vmetrics.ascent);

    let glyph = font.glyph(ch).scaled(scale);
    let hmetrics = glyph.h_metrics();

    let image_width = hmetrics.advance_width.ceil() as usize;
    let image_height = (vmetrics.ascent - vmetrics.descent).ceil() as usize;

    println!(
        "image_width: {}, image_height: {}",
        image_width, image_height
    );

    let glyph = glyph.positioned(offset);
    let bb = glyph.pixel_bounding_box();
    println!("bb: {:?}", bb);
    let mut ret = Vec::new();
    ret.resize(image_width * image_height, 0.0f32);
    if let Some(bb) = bb {
        glyph.draw(|x, y, v| {
            let x = x as i32 + bb.min.x;
            let y = y as i32 + bb.min.y;

            // there are some characters that have big pixel_bounding_box
            // e.g. underscore '_'
            if x < 0 || x >= image_width as i32 || y < 0 || y >= image_height as i32 {
                return;
            }

            ret[y as usize * image_width + x as usize] = v;
        });
    }
    ret
}

pub fn render_str<'a, 'b>(
    font: &Font<'a>,          // the font to be used
    s: &'b str,               // the string to be rendered
    line_height: usize,       // the height of the line, in the number of characters to be rendered
    char_offset_factor: f32,  // the factor to multiply the advance width of each character
    threshold: f32,           // threshold for binarization
    cursor_aspect_ratio: f32, // the ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
    max_width: Option<usize>, // the maximum width of the rendered image
) -> Result<Vec<Vec<bool>>, char> // returns None if there is a single character that cannot be fit into max_width
{
    // the ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
    let scale = Scale {
        x: line_height as f32 * cursor_aspect_ratio,
        y: line_height as f32,
    };
    let vmetrics = font.v_metrics(scale);
    let image_height = (vmetrics.ascent - vmetrics.descent).ceil() as usize;

    let mut ret_lines = Vec::new();

    // col-major
    let mut cur_line = Vec::new();
    for ch in s.chars() {
        let char_image = render_char(font, ch, line_height);
        let char_width = char_image.len() / image_height;

        // check this single character exceeds max_width
        if let Some(max_width) = max_width {
            if char_width >= max_width {
                return Err(ch);
            }
        }

        // check if this character can be fit into current line
        let char_offset = (char_width as f32 * char_offset_factor).round() as i32;
        let x_offset = if cur_line.len() == 0 {
            0
        } else {
            let x_offset = (cur_line.len() as i32 + char_offset) as usize;
            if let Some(max_width) = max_width {
                if x_offset + char_width >= max_width {
                    // this character cannot be fit into current line
                    // push the current line to ret_lines
                    ret_lines.push(cur_line);
                    cur_line = Vec::new();
                    0
                } else {
                    x_offset
                }
            } else {
                x_offset
            }
        };

        let mut cols = Vec::new();
        cols.resize(image_height, 0.0f32);
        cur_line.resize(x_offset as usize + char_width as usize, cols);

        for x in 0..char_width {
            for y in 0..image_height {
                let v = char_image[y * char_width + x];
                let v = (1.0 - cur_line[x + x_offset][y]) * (1.0 - v);
                cur_line[x + x_offset][y] = 1.0 - v;
            }
        }
    }
    if cur_line.len() > 0 {
        ret_lines.push(cur_line);
    }

    let mut ret = Vec::new();
    ret.reserve(ret_lines.len());
    for big_line in ret_lines.into_iter() {
        // big_line is a Vec<Vec<f32>>, column-major

        let image_width = big_line.len();

        // flatten the big_line into a Vec<f32>, row-major and binarize
        let mut flattened = Vec::new();
        flattened.reserve(image_width * image_height);
        for y in 0..image_height {
            for x in 0..image_width {
                flattened.push(big_line[x][y] > threshold);
            }
        }

        ret.push(flattened);
    }

    Ok(ret)
}
