mod app;
mod js;

use std::rc::Rc;
use std::sync::mpsc;
use async_std::prelude::StreamExt;
use leptos::*;
use leptos::mount_to_body;
use leptos::prelude::*;
use image::{DynamicImage, ImageError, ImageFormat};
use base64::{Engine};
use base64::engine::general_purpose;
use image::imageops::FilterType;
use js_sys::Uint8Array;
use leptos::{IntoView};
use leptos_mview::mview;
use tar::{Builder, Header};
use uuid::Uuid;
use crate::app::{convert_image, App};
use crate::js::downloadFile;


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
    result: Vec<u8>,
    
    in_file: FileInfo,
    out_file: Option<FileInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileInfo {
    name: String,
    file_type: ImageFormat,
    metadata: Option<u8>, // TODO
    bytes: Vec<u8>
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            name: String::default(),
            file_type: ImageFormat::Png,
            metadata: None,
            bytes: Vec::default(),
        }
    }
}



#[derive(Clone, Default)]
struct AppState {
    input_files: RwSignal<Vec<DisplayImage>>,
    queued_files: RwSignal<Vec<DisplayImage>>,
    output_files: RwSignal<Vec<DisplayImage>>,
}

impl AppState {
    fn detect_format(bytes: &[u8]) -> Option<ImageFormat> {
        let format = image::guess_format(bytes).ok()?;
        Some(format)
    }

    pub fn generate_thumbnail_str(file_info: FileInfo) -> String {
        let img = image::load_from_memory(&file_info.bytes)
            .expect("Failed to load image from memory");

        let thumbnail = img.resize(64, 64, FilterType::Lanczos3);

        let mut buffer = Vec::new();
        thumbnail.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
            .expect("Failed to write image to buffer");

        let base64 = general_purpose::STANDARD.encode(&buffer);
        format!("data:image/png;base64,{}", base64)
    }

    pub fn queue_selected(&self, output_format: ImageFormat) {
        self.queued_files.update(|queued| {
            let mut selected: Vec<DisplayImage> = self.input_files.get().iter().filter(|img| img.is_selected.get()).cloned().collect();
            selected.iter_mut().for_each(|img| img.out_filetype = Some(output_format));
            queued.extend(selected);
            self.input_files.update(|queue| queue.retain(|image| !image.is_selected.get()));
        });
    }

    pub fn download_selected(&self) {
        if self.output_files.get().is_empty() {
            return;
        }

        let buffer = std::io::Cursor::new(Vec::new());
        let mut a = Builder::new(buffer);

        self.output_files.get()
            .iter()
            .filter(|img| img.is_selected.get())
            .for_each(|img| {
                let old_termination = format!(".{}", img.in_filetype);

                // strip file ext
                let file_name = if img.name.ends_with(&old_termination) {
                    img.name.strip_suffix(&old_termination)
                        .unwrap_or(&img.name)
                        .to_string()
                } else {
                    img.name.to_string()
                };

                // Truncate file name to a maximum of 64 characters
                let file_name = file_name.chars().take(64).collect::<String>();

                let mut header = Header::new_gnu();
                header.set_path(format!("{file_name}.{}", img.out_filetype.unwrap()
                    .extensions_str()[0])).unwrap(); // TODO can add support for other terminations in additional settings
                header.set_size(img.result.len() as u64);
                header.set_mode(0o644);
                // header.set_metadata()
                header.set_cksum();

                a.append(&header, img.result.as_slice()).unwrap()
            });

        // Get the TAR data from the buffer
        let tar_data = a.into_inner().unwrap().into_inner();

        // Convert tar data to Uint8Array
        let js_data = Uint8Array::from(tar_data.as_slice());

        downloadFile("output.tar", js_data.into());
    }


    // pub async fn step_queue(&self, mut rx: mpsc::Receiver<()>) {
    //     while let Some(_) = rx.next().await {
    //         let queued = self.queued_files;
    //
    //         if let Some(mut file) = queued.get().pop() {
    //             let out_type = file.out_filetype.clone().unwrap();
    //             let encoded = convert_image(file.image.clone(), out_type).await;
    //
    //             file.result = encoded.expect("Failed to process a picture");
    //
    //             self.output_files.update(|output| {
    //                 output.push(file);
    //             });
    //         }
    //     }
    // }

}

fn generate_sample_image(img: &DynamicImage, buffer: &mut Vec<u8>) -> String {
    buffer.clear(); // Clear the buffer for reuse

    // Resize the image
    let resized = img.resize(64, 64, FilterType::Lanczos3);

    // Create a Cursor wrapping the buffer
    let mut cursor = std::io::Cursor::new(buffer);

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

impl IntoView for DisplayImage {
    fn into_view(self) -> View {
        let name = self.name.clone();
        let is_completed = self.is_completed;
        let preview = self.preview.clone();
        let completed_time = self.time_completed.clone();

        let is_selected = self.is_selected.clone();

        let conversion_str = match &self.out_filetype {
            None => format!("{}", self.in_filetype),
            Some(out_ext) => format!("{} -> {}",
                                     self.in_filetype, out_ext.extensions_str()[0]),
        };

        let on_checkbox=move |ev| {
            is_selected.set(event_target_checked(&ev))
        };

        let on_clicked = move |_| {
            is_selected.set(!is_selected.get());
        };



        let finish_time = completed_time.unwrap_or_else(|| String::new());

        let element =
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
        };

        element.into_view()
    }
}

fn main() -> Result<(), ImageError> {
    console_error_panic_hook::set_once();
    mount_to_body(move|| view! {
        <App />
    });


    Ok(())
}
