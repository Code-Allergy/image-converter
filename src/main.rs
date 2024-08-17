mod app;

use std::fs::File;
use std::io;
use std::io::{Cursor, Read};
use leptos::*;
use leptos::mount_to_body;
use leptos::prelude::*;
use image::{DynamicImage, ImageError, ImageFormat};
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter, Engine};
use base64::engine::general_purpose;
use image::imageops::FilterType;
use leptos::{component, view, IntoView};
use leptos_mview::mview;
use uuid::Uuid;
use web_sys::MouseEvent;
use crate::app::{App};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DisplayImage {
    id: String,
    is_completed: bool,
    is_selected: RwSignal<bool>,
    name: String,
    in_filetype: &'static str,
    out_filetype: Option<&'static str,>,
    time_completed: Option<String>, // FOR NOW this is string todo
    image: DynamicImage
}



#[derive(Clone, Default)]
struct AppState {
    input_files: RwSignal<Vec<DisplayImage>>,
    count: i32,
}

// fn image_to_data_url(image: &DynamicImage) -> String {
//     // Convert the DynamicImage to a PNG format
//     let mut buf = Vec::new();
//     let mut out_buff = String::new();
//     let encoder = image::codecs::png::PngEncoder::new(&mut buf);
//     encoder.encode_image(image).expect("Failed to encode image");
//
//     let mut b64_encoder = EncoderWriter::new(&mut out_buff, &STANDARD);
//     io::copy(&mut io::stdin(), &mut b64_encoder).expect("Failed to base64 encode image");
//
//     // Format the base64 string into a data URL
//     format!("data:image/png;base64,{}", out_buff)
// }

fn generate_sample_image(img: &DynamicImage) -> String {
    // Create a buffer to store the image data
    let mut buffer = Cursor::new(Vec::new());

    // Write the image to the buffer in the specified format
    let resized = img.resize(64, 64, FilterType::Lanczos3);
    resized.write_to(&mut buffer, ImageFormat::Png).expect("Failed to write image to buffer");

    // Get the bytes from the buffer
    let bytes = buffer.into_inner();


    // Encode the bytes to base64
    let base64 = general_purpose::STANDARD.encode(bytes);


    format!("data:image/png;base64,{}", base64)
}

pub fn generate_unique_key() -> String {
    Uuid::new_v4().to_string()
}


impl DisplayImage {
    pub fn render(&self) -> impl IntoView {
        let name = self.name.clone();
        let completed_time = self.time_completed.clone();
        let is_completed = self.is_completed;

        let is_selected = self.is_selected.clone();

        let conversion_str = match self.out_filetype {
            None => format!("{}", self.in_filetype),
            Some(out_ext) => format!("{} -> {}", self.in_filetype, out_ext),

        };

        let on_checkbox=move |ev| {
            is_selected.set(event_target_checked(&ev))
        };

        let on_clicked = move |mv: MouseEvent| {
            let invert = !is_selected.get();
            is_selected.set(invert);
        };

        let image_str = generate_sample_image(&self.image);
        let finish_time = completed_time.unwrap_or_else(|| String::new());

        mview! {
            div class="flex flex-row align-middle w-full h-20 hover:bg-blue-700" on:click={on_clicked} {
                Show when=[!is_completed] fallback=[view! {""}] {
                    div class="flex items-center justify-center pl-2" {
                        label class="custom-checkbox inline-flex" {
                            input type="checkbox" checked={is_selected.get()} on:input={on_checkbox}; {}
                        }
                    }


                }
                img src={image_str} class="m-2 h-16 w-16 bg-red-800" {

                }
                div class="mt-1 w-full h-full overflow-hidden" {
                    p {{name}}
                    p {{conversion_str}}
                    p {{finish_time}}
                    hr class="w-full border-t border-gray-300";
                }
            }
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, ImageError> {
        let mut file = File::open(filename)?;
        let mut buffer = vec![];
        let bytes_read = file.read_to_end(&mut buffer)?;
        let format = image::guess_format(&buffer)?;
        let img = image::load_from_memory(&buffer)?;

        Ok(DisplayImage {
            id: generate_unique_key(),
            is_completed: false,
            is_selected: create_rw_signal(false),
            name: filename.to_string(),
            in_filetype: format.extensions_str()[0],
            out_filetype: None,
            time_completed: None,
            image: img,
        })
    }

    pub fn from_bytes(filename: &str, bytes: &[u8]) -> Result<Self, ImageError> {
        let format = image::guess_format(bytes)?;
        let img = image::load_from_memory(bytes)?;

        Ok(DisplayImage {
            id: generate_unique_key(),
            is_completed: false,
            is_selected: create_rw_signal(false),
            name: filename.to_string(),
            in_filetype: format.extensions_str()[0],
            out_filetype: None,
            time_completed: None,
            image: img,
        })
    }
}

fn main() -> Result<(), ImageError> {
    console_error_panic_hook::set_once();
    mount_to_body(move|| view! {
        <App />
    });


    Ok(())
}
