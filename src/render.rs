use std::io::Cursor;
use image::io::Reader as ImageReader;
use fantoccini::ClientBuilder;
use urlencoding::encode;
use std::error::Error;

pub async fn get_screenshot(html: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut capabilities = serde_json::map::Map::new();
    let options = serde_json::json!({ "args": ["--headless","--width=1024","--height=4096"] });
    capabilities.insert("moz:firefoxOptions".to_string(), options.clone());
    let c = ClientBuilder::native().capabilities(capabilities).connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    
    let data_encoded = encode(&html);
    let uri = format!("data:text/html,{}",data_encoded);
    c.goto(&uri).await?;
    let png_data = c.screenshot().await?;
    c.close().await?;

    
    return autocrop_and_save(png_data);
}

fn autocrop_and_save(png_data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>{
    let img = ImageReader::new(Cursor::new(png_data)).with_guessed_format()?.decode()?.into_rgba8();
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
    let mut c = Cursor::new(Vec::new());
    imgout.write_to(&mut c, image::ImageOutputFormat::Png)?;
    return Ok(c.get_ref().clone());
    //imgout.save(filename)?;
}