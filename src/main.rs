
use std::io::Cursor;
use image::io::Reader as ImageReader;
use fantoccini::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut capabilities = serde_json::map::Map::new();
    let options = serde_json::json!({ "args": ["--headless","--width=1024","--height=4096"] });
    capabilities.insert("moz:firefoxOptions".to_string(), options.clone());
    let c = ClientBuilder::native().capabilities(capabilities).connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let png_data = c.screenshot().await?;

    let img = ImageReader::new(Cursor::new(png_data)).with_guessed_format().unwrap().decode().unwrap().into_rgba8();
    autocrop_and_save(img, "out3.png");
    c.close().await?;
    return Ok(());
}

fn autocrop_and_save(img: image::RgbaImage, filename: &str){
    let dimensions = img.dimensions();
    println!("{},{}",dimensions.0,dimensions.1);
    let mut left = dimensions.0;
    let mut right = 0;
    let mut top = dimensions.1;
    let mut bottom = 0;
    let mut tl_color: Option<image::Rgba<u8>> = None;
    let mut br_color: Option<image::Rgba<u8>> = None;
    for (x, y, pixel) in img.enumerate_pixels() {
        if let Some(pix) = tl_color {
            if pix != *pixel{
                if x < left {
                    left = x;
                }
                if y < top {
                    top = y;
                }
            }
        } else {
            tl_color = Some(pixel.clone());
        }
    }
    //Pain, no double ended iterator :(
    for x in (0..dimensions.0).rev() {
        for y in (0..dimensions.1).rev() {
            let pixel = img.get_pixel(x,y);
            if let Some(pix) = br_color {
                if pix != *pixel{
                    if x > right {
                        right = x;
                    }
                    if y > bottom {
                        bottom = y;
                    }
                }
            } else {
                br_color = Some(pixel.clone());
            }
        }
    }
    let imgout = image::imageops::crop_imm(&img,left,top,right,bottom).to_image();
    println!("tl:{:?},br:{:?},top:{:?},bottom:{:?},left:{:?},right:{:?}",tl_color,br_color,top,bottom,left,right);
    imgout.save(filename).unwrap();
}