use std::io::{Cursor, Write};
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Pixel, Rgb, RgbImage};
use image::io::Reader as ImageReader;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum ConversionMode {
    NoConversion,
    YCbCr,
    HSV,
    Lab,
}


fn write_to_p6(image: DynamicImage, buffer: &mut Vec<u8>) -> std::io::Result<()> {
    writeln!(buffer, "P6")?;
    writeln!(buffer, "{} {}", image.width(), image.height())?;
    writeln!(buffer, "255")?;

    for pixel in image.pixels() {
        buffer.write_all(&pixel.2.0[0..3])?;
    }

    Ok(())
}


fn rgb2ycbcr(image: RgbImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let conversion_matrix = vec![
        vec![0.299, 0.587, 0.114],
        vec![-0.168736, -0.331264, 0.5],
        vec![0.5, -0.418688, -0.081312],
    ];

    let new_image = ImageBuffer::from_par_fn(width, height, |x, y| {
        let pixel = image.get_pixel(x, y);
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let y = (conversion_matrix[0][0] * r
            + conversion_matrix[0][1] * g
            + conversion_matrix[0][2] * b)
            * 255.0;
        let cb = (conversion_matrix[1][0] * r
            + conversion_matrix[1][1] * g
            + conversion_matrix[1][2] * b
            + 0.5)
            * 255.0;
        let cr = (conversion_matrix[2][0] * r
            + conversion_matrix[2][1] * g
            + conversion_matrix[2][2] * b
            + 0.5)
            * 255.0;

        Rgb([y as u8, cb as u8, cr as u8])
    });

    DynamicImage::from(new_image)
}

fn rgb2hsv(image: RgbImage) -> DynamicImage {
    let (width, height) = image.dimensions();

    let new_image = ImageBuffer::from_par_fn(width, height, |x, y| {
        let pixel = image.get_pixel(x, y);
        let min_ch = *pixel.channels().iter().min().unwrap() as f32 / 255.0;
        let max_ch = *pixel.channels().iter().max().unwrap() as f32 / 255.0;
        let diff = max_ch - min_ch;

        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let mut v = max_ch;

        let mut s;
        if max_ch != 0.0 {
            s = diff / max_ch;
        } else {
            s = 0.0;
        }

        let mut h;
        if diff == 0.0 {
            h = 0.0;
        } else if v == r {
            h = 60.0 * (g - b) / diff;
        } else if v == g {
            h = (60.0 * (b - r) / diff) + 120.0;
        } else {
            h = (60.0 * (r - g) / diff) + 240.0;
        }

        if h < 0.0 {
            h = h + 360.0;
        }

        v = v * 255.0;
        s = s * 255.0;
        h = h / 360.0 * 255.0;

        Rgb([h as u8, s as u8, v as u8])
    });

    DynamicImage::from(new_image)
}

pub fn do_conversion(image_raw: Vec<u8>, conversion_mode: ConversionMode) -> Result<Vec<u8>, String> {
    let reader = match ImageReader::new(Cursor::new(image_raw)).with_guessed_format() {
        Ok(reader) => reader,
        Err(error_message) => return Err(String::from(format!("Internal error. Could not read the image: {error_message}")))
    };

    let image_format = match reader.format() {
        Some(format) => format,
        _ => return Err(String::from("Not supported image format"))
    };

    let image_rgb = match reader.decode() {
        Ok(image) => image.to_rgb8(),
        _ => return Err(String::from("Internal error. Could not decode image")),
    };

    let output_image;
    match conversion_mode {
        ConversionMode::NoConversion => {
            output_image = DynamicImage::from(image_rgb);
        }
        ConversionMode::YCbCr => {
            output_image = rgb2ycbcr(image_rgb);
        }
        ConversionMode::HSV => {
            output_image = rgb2hsv(image_rgb);
        }
        ConversionMode::Lab => {
            output_image = DynamicImage::from(image_rgb);
        }
    };

    let mut image_buffer = Vec::new();
    match image_format {
        ImageFormat::Pnm => {
            match write_to_p6(output_image, &mut image_buffer) {
                Ok(()) => (),
                Err(error) => return Err(String::from(format!("{error:?}")))
            }
        }
        _ => {
            match output_image.write_to(&mut Cursor::new(&mut image_buffer), image_format) {
                Ok(()) => (),
                Err(error_message) => return Err(format!("{error_message:?}"))
            };
        }
    }

    Ok(image_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_ycbcr() -> Result<(), String> {
        let test_image = RgbImage::from_raw(1, 1, vec![100, 50, 200]).unwrap();

        let out_image = rgb2ycbcr(test_image);

        let pixel = out_image.as_bytes();
        let pixel_matlab = [82, 194, 140]; // from matlab

        match pixel == pixel_matlab {
            true => Ok(()),
            false => Err(String::from("Conversion YCbCr has bugs")),
        }
    }

    #[test]
    fn conversion_hsv() -> Result<(), String> {
        let test_image = RgbImage::from_raw(1, 1, vec![100, 50, 200]).unwrap();

        let out_image = rgb2hsv(test_image);

        let pixel = out_image.as_bytes();
        let pixel_matlab = [184, 191, 200]; // from matlab

        match pixel == pixel_matlab {
            true => Ok(()),
            false => Err(String::from("Conversion HSV has bugs")),
        }
    }
}
