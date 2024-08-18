mod app;
mod js;
use std::fs::File;
use std::io::{Cursor, Read};
use leptos::*;
use leptos::mount_to_body;
use leptos::prelude::*;
use image::{DynamicImage, ImageError, ImageFormat};
use base64::{Engine};
use base64::engine::general_purpose;
use image::imageops::FilterType;
use leptos::{IntoView};
use leptos_mview::mview;
use uuid::Uuid;
use web_sys::MouseEvent;
use crate::app::App;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DisplayImage {
    id: String,
    is_completed: bool,
    is_selected: RwSignal<bool>,
    name: String,
    preview: String,
    in_filetype: &'static str,
    out_filetype: Option<ImageFormat>,
    time_completed: Option<String>, // FOR NOW this is string todo
    image: DynamicImage,
    result: Vec<u8>
}



#[derive(Clone, Default)]
struct AppState {
    input_files: RwSignal<Vec<DisplayImage>>,
    queued_files: RwSignal<Vec<DisplayImage>>,
    output_files: RwSignal<Vec<DisplayImage>>,
    count: i32,
}

fn generate_sample_image(img: &DynamicImage, buffer: &mut Vec<u8>) -> String {
    buffer.clear(); // Clear the buffer for reuse

    // Resize the image
    let resized = img.resize(64, 64, FilterType::Lanczos3);

    // Create a Cursor wrapping the buffer
    let mut cursor = Cursor::new(buffer);

    // Write the image to the cursor in JPEG format with 85% quality
    resized.write_to(&mut cursor, ImageFormat::Png)
        .expect("Failed to write image to buffer");

    // Get the inner buffer from the cursor
    let buffer = cursor.into_inner();

    // Encode the bytes to base64
    let base64 = general_purpose::STANDARD.encode(buffer);

    // Return the data URL
    format!("data:image/png;base64,{}", base64)
}

pub fn generate_unique_key() -> String {
    Uuid::new_v4().to_string()
}


impl DisplayImage {
    pub fn render(&self) -> impl IntoView {
        let name = self.name.clone();
        let is_completed = self.is_completed;
        let preview = self.preview.clone();
        let completed_time = self.time_completed.clone();


        let is_selected = self.is_selected.clone();

        let conversion_str = match &self.out_filetype {
            None => format!("{}", self.in_filetype),
            Some(out_ext) => format!("{} -> {}", self.in_filetype, out_ext.extensions_str()[0]),
        };

        let on_checkbox=move |ev| {
            is_selected.set(event_target_checked(&ev))
        };

        let on_clicked = move |mv: MouseEvent| {
            let invert = !is_selected.get();
            is_selected.set(invert);
        };



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
                img src={preview} class="m-2 h-16 w-16 bg-red-800" {

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
        file.read_to_end(&mut buffer)?;
        let format = image::guess_format(&buffer)?;
        let img = image::load_from_memory(&buffer)?;

        Ok(DisplayImage {
            id: generate_unique_key(),
            is_completed: false,
            is_selected: create_rw_signal(false),
            name: filename.to_string(),
            preview: String::new(),
            in_filetype: format.extensions_str()[0],
            out_filetype: None,
            time_completed: None,
            image: img,
            result: vec![],
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
            preview: String::new(),
            in_filetype: format.extensions_str()[0],
            out_filetype: None,
            time_completed: None,
            image: img,
            result: vec![],

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
