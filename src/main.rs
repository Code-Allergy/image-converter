mod app;

use std::fs::File;
use std::io;
use std::io::{Error, Read};
use leptos::*;
use leptos::mount_to_body;
use leptos::prelude::*;
use image::{DynamicImage, ImageError, ImageFormat, ImageReader};


use leptos::{component, view, IntoView};
use leptos_mview::mview;
use crate::app::{add_image, App};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DisplayImage {
    id: u32,
    is_completed: bool,
    name: String,
    in_filetype: &'static str,
    out_filetype: Option<&'static str,>,
    time_completed: Option<String> // FOR NOW this is string todo
    // image: DynamicImage
}



#[derive(Clone, Default)]
struct AppState {
    input_files: RwSignal<Vec<DisplayImage>>,
    count: i32,
}

impl DisplayImage {
    pub fn render(&self) -> impl IntoView {
        let (checked, set_checked) = create_signal(false);

        let name = self.name.clone();
        let completed_time = self.time_completed.clone();
        let is_completed = self.is_completed;



        let conversion_str = match self.out_filetype {
            None => format!("{}", self.in_filetype),
            Some(out_ext) => format!("{} -> {}", self.in_filetype, out_ext),

        };




        let finish_time = completed_time.unwrap_or_else(|| String::new());

        mview! {
            div class="flex flex-row align-middle w-full h-20 hover:bg-blue-700" {
                Show
                    when=[!is_completed]
                    fallback=[view! {""}] {
                    label class="custom-checkbox inline-flex items-center" {
                        input type="checkbox" on:input={move |ev| {
                            set_checked.set(event_target_checked(&ev))
                        }}; {}

                    }

                }
                div class="m-2 h-16 w-16 bg-red-800" {

                }
                div class="mt-1 w-full h-full" {
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
            id: 0,
            is_completed: false,
            name: filename.to_string(),
            in_filetype: format.extensions_str()[0],
            out_filetype: None,
            time_completed: None,
        })
    }
}







#[component]
fn ImageGallery() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");


    let images = app_state.input_files;

    view! {
        <div class="h-5/6 w-full bg-gray-200 overflow-auto">
            <For
                each=move || images.get()
                key=|display_img| display_img.id.clone()
                children=move |img| {
                    {img.render()}
                }
            />
        </div>
    }
}






fn main() -> Result<(), ImageError> {
    console_error_panic_hook::set_once();
    mount_to_body(move|| view! {
        <App />
    });


    Ok(())
}
