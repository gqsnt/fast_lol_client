use std::path::Path;
use iced::advanced::Widget;
use iced::Length;
use iced::widget::{container, image, Image};
use iced::widget::image::{Handle, Viewer};
use serde_json::Value;
use crate::AppResult;

pub mod process_info;

pub fn save_json_to_file(name: &str, json: &Value) -> std::io::Result<()> {
    let temp = std::path::Path::new("temp");
    if !temp.exists() {
        std::fs::create_dir_all(temp)?;
    }
    let file = std::fs::File::create(temp.join(name))?;
    serde_json::to_writer_pretty(file, json)?;
    Ok(())
}


pub fn draw_image(image:Option<Handle>, width:Length, height:Length) -> Option<Image<Handle>>{
    image.map(|handle| {
        Image::new(handle)
            .width(width)
            .height(height)
    })
}
