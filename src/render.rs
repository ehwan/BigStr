use rusttype::{Font, Scale};

pub fn render_char<'a>(font: &Font<'a>, ch: char, line_height: usize, threshold: f32) -> Vec<bool> {
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

    let glyph = glyph.positioned(offset);
    let bb = glyph.pixel_bounding_box();
    if let Some(bb) = bb {
        let mut ret = Vec::new();
        ret.resize(image_width * image_height, false);

        glyph.draw(|x, y, v| {
            let x = (x as i32 + bb.min.x) as usize;
            let y = (y as i32 + bb.min.y) as usize;

            ret[y * image_width + x] = v > threshold;
        });

        ret
    } else {
        Vec::new()
    }
}

pub fn render_str<'a, 'b>(
    font: &Font<'a>,          // the font to be used
    s: &'b str,               // the string to be rendered
    line_height: usize,       // the height of the line, in the number of characters to be rendered
    char_offset_factor: f32,  // the factor to multiply the advance width of each character
    threshold: f32,           // threshold for binarization
    cursor_aspect_ratio: f32, // the ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
) -> Vec<bool> {
    // the ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
    let scale = Scale {
        x: line_height as f32 * cursor_aspect_ratio,
        y: line_height as f32,
    };
    let vmetrics = font.v_metrics(scale);
    let font_offset = rusttype::point(0.0, vmetrics.ascent);

    let image_height = (vmetrics.ascent - vmetrics.descent).ceil() as usize;

    let mut image_width: i32 = 0;
    for ch in s.chars() {
        let glyph = font.glyph(ch).scaled(scale);
        let hmetrics = glyph.h_metrics();

        let glyph_width = hmetrics.advance_width.ceil() as i32;
        let char_offset = (hmetrics.advance_width * char_offset_factor).round() as i32;
        if image_width == 0 {
            image_width = glyph_width;
        } else {
            image_width += glyph_width + char_offset;
        }
    }

    let mut image = Vec::new();
    image.resize(image_height * image_width as usize, 0.0f32);

    let mut cur_width: i32 = 0;
    for ch in s.chars() {
        let glyph = font.glyph(ch).scaled(scale);
        let hmetrics = glyph.h_metrics();

        let char_offset = (hmetrics.advance_width * char_offset_factor).round() as i32;

        let glyph = glyph.positioned(font_offset);
        let bb = glyph.pixel_bounding_box();

        let x_offset = if cur_width == 0 {
            0
        } else {
            cur_width + char_offset
        };

        // skip characters that have no bounding box (e.g. space)
        if let Some(bb) = bb {
            glyph.draw(|x, y, v| {
                let x = (x as i32 + bb.min.x + x_offset) as usize;
                let y = (y as i32 + bb.min.y) as usize;

                let old = image[y * image_width as usize + x];
                let new = 1.0 - (1.0 - old) * (1.0 - v); // alpha blending

                image[y * image_width as usize + x] += new;
            });
        }

        let glyph_width = hmetrics.advance_width.ceil() as i32;
        if cur_width == 0 {
            cur_width = glyph_width;
        } else {
            cur_width += glyph_width + char_offset;
        }
    }

    image.iter().copied().map(|v| v > threshold).collect()
}
