use std::cell::RefCell;
use std::io::{Cursor};
use std::time::Duration;
use image::{DynamicImage, EncodableLayout, ExtendedColorType, ImageEncoder, ImageFormat};

use leptos_mview::mview;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::{Closure};
use web_sys::{Event, File, FileList, HtmlInputElement};
use crate::{generate_sample_image, generate_unique_key, AppState, DisplayImage};
use wasm_bindgen::prelude::*;

use leptos::{component, create_rw_signal, create_signal, event_target_value, provide_context, use_context, view, Callable, Callback, For, IntoView, RwSignal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate};
use wasm_bindgen_futures::spawn_local;

pub fn convert_image(img: DynamicImage, format: ImageFormat) -> Result<Vec<u8>, ()> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    match format {
        ImageFormat::Png => {
            let encoder = image::codecs::png::PngEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Jpeg => {
            let encoder = image::codecs::jpeg::JpegEncoder::new(&mut cursor);
            let img = img.to_rgb8();
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::Rgb8, // TODO handle removing alpha for jpeg
            ).unwrap();
        },
        ImageFormat::Gif => {
            let mut encoder = image::codecs::gif::GifEncoder::new(&mut cursor);
            encoder.encode(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        // ImageFormat::WebP => {
        //     let encoder = image::codecs::webp::WebPEncoder::new(&mut cursor);
        //     encoder.write_image(
        //         img.as_bytes(),
        //         img.width(),
        //         img.height(),
        //         ExtendedColorType::from(img.color()),
        //     ).unwrap();
        // },
        ImageFormat::Pnm => {
            let encoder = image::codecs::pnm::PnmEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Tiff => {
            let encoder = image::codecs::tiff::TiffEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Tga => {
            let encoder = image::codecs::tga::TgaEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Bmp => {
            let encoder = image::codecs::bmp::BmpEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Ico => {
            let encoder = image::codecs::ico::IcoEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Hdr => {
            let encoder = image::codecs::hdr::HdrEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::OpenExr => {
            let encoder = image::codecs::openexr::OpenExrEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Farbfeld => {
            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Avif => {
            let encoder = image::codecs::avif::AvifEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        ImageFormat::Qoi => {
            let encoder = image::codecs::qoi::QoiEncoder::new(&mut cursor);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                ExtendedColorType::from(img.color()),
            ).unwrap();
        },
        _ => {}
    };
//
    Ok(buffer)
}




#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState { input_files: Default::default(), queued_files: Default::default(),
        output_files: Default::default()};

    provide_context(app_state.clone());
    let tester_resolver = spawn_local(async move {
        loop {
            let queued = app_state.queued_files;
            let queued_items = app_state.queued_files.get_untracked();


            while !queued_items.is_empty() {
                queued.update(|files| {
                    let output_items = app_state.output_files;
                    let mut file = files.pop().unwrap();
                    let out_type = file.out_filetype.clone().unwrap();
                    let encoded = convert_image(file.image.clone(), out_type);

                    // process the image now.
                    file.result = encoded.expect("Failed to process a picture to PNG");

                    output_items.clone().update(|output_item| output_item.push(file));
                })

            }
            async_std::task::sleep(Duration::from_micros(1000)).await;
        }
    });

    mview! {
        div class="flex w-screen h-screen bg-gray-100 justify-center items-center" {
            div class="xl:w-3/4 xl:h-3/4 h-full w-full lg:rounded-lg bg-gray-500" {
                div class="flex h-full flex-col xl:flex-row"{
                    div class="flex lg:basis-1/3 h-full flex-col" {
                        UploadedImagesContainer;
                    }
                    div class="h-full lg:basis-1/3 h-full flex flex-col" {
                        ConversionOptionsPanel;
                        QueuedImagesContainer;

                    }
                    div class="h-full lg:basis-1/3 flex flex-col" {
                        OutputImagesContainer;

                    }
                    DownloadButton;
                }
            }
        }
    }
}

#[component]
fn DownloadButton() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");

    mview! {
        button on:click={ move |_| state.download_selected() } {"Download"}
    }
}


#[component]
pub fn ConversionOptionsPanel() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");

    let (output_format, set_output_format) = create_signal(ImageFormat::Png); // png is first selected

    mview! {
        div class="flex items-center justify-center h-full"{
            div class="flex flex-col items-center justify-center h-5/6 w-full bg-primary h-full text-sm" {
                FormatSelector on_change={move |format| set_output_format.set(format)};
                button class="px-4 py-2 bg-button w-full lg:h-24 lg:w-1/4 bg-button text-sm" on:click={move |_| app_state.queue_selected(output_format.get())} {
                    "Convert"
                }
            }
        }
    }
}


#[component]
fn FormatSelector(
    #[prop(into)] on_change: Callback<ImageFormat>
) -> impl IntoView {
    let update_format = move |ev| {
        let format = event_target_value(&ev);
        let image_format = match format.as_str() {
            "PNG" => ImageFormat::Png,
            "BMP" => ImageFormat::Bmp,
            "GIF" => ImageFormat::Gif,
            "HDR" => ImageFormat::Hdr,
            "ICO" => ImageFormat::Ico,
            "JPEG" => ImageFormat::Jpeg,
            "EXR" => ImageFormat::OpenExr,
            "PNM" => ImageFormat::Pnm,
            "TGA" => ImageFormat::Tga,
            "TIFF" => ImageFormat::Tiff,
            "WEBP" => ImageFormat::WebP,
            _ => ImageFormat::Png, // Default to PNG if unknown, TODO error handling later, should inform & ignore
        };
        on_change.call(image_format);
    };

    view! {
        <select class="w-full" id="format-selector" name="format" on:change=update_format>
            <option value="PNG">"PNG"</option>
            <option value="BMP">"BMP"</option>
            <option value="GIF">"GIF"</option>
            <option value="HDR">"HDR"</option>
            <option value="ICO">"ICO"</option>
            <option value="JPEG">"JPEG"</option>
            <option value="EXR">"EXR"</option>
            <option value="PNM">"PNM"</option>
            <option value="TGA">"TGA"</option>
            <option value="TIFF">"TIFF"</option>
            <option value="WEBP">"WEBP"</option>
        </select>
    }
}


#[component]
pub fn UploadedImagesContainer() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");

    mview! {
        div class="h-full flex-col flex" {
            h1 class="lg:text-xl text-center" {"Uploaded"}
            div class="h-40 w-full" {
                ImageUploader;
                ImageContainer id="upload-images" source={app_state.input_files};
            }
        }

    }
}

#[component]
pub fn QueuedImagesContainer() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");

    mview! {
        div class="h-full flex flex-col justify-center" {
            h1 class="lg:text-xl text-center bg-secondary" {"Queued"}
            div class="h-40 w-full" {
                ImageContainer id="upload-images" source={app_state.queued_files};
            }

        }

    }
}

#[component]
pub fn OutputImagesContainer() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");

    mview! {
        div class="flex-grow w-full" {
            h1 class="lg:text-xl text-center" {"Finished"}
            ImageContainer id="upload-images" source={state.output_files};
        }
    }
}





#[component]
pub fn ImageUploader() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");

    let (file_count, set_file_count) = create_signal(0);

    let app_clone = state.clone();
    let on_files_change = move |ev: Event| {
        let app_state = state.clone();
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files() {
            process_files(file_list);
        }
    };

    view! {
        <div class="flex">
            <label class="inline-flex grow items-center px-4 py-2 \
                bg-button rounded-md shadow-sm cursor-pointer hover:bg-gray-50">
              <input type="file" accept="image/*" class="hidden" on:change=on_files_change multiple />
              <span class="text-sm w-full text-center">Choose Files</span>
            </label>
        </div>
    }
}
#[component]
pub fn ImageContainer(id: &'static str, source: RwSignal<Vec<DisplayImage>>) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");

    let (all_selected, set_all_selected) = create_signal(false);
    let select_state = state.clone();

    let select_all_toggle = move |mc| {
        let select_state = !all_selected.get();
        source.update(|files| {
            files.iter_mut().for_each(|img| {
                img.is_selected.set(select_state);
            });
        });
    };


    view! {
        <div id={id} class="h-full w-full bg-primary overflow-auto border-2">
            <div class="flex flex-row">
                <input type="checkbox" class="m-2 custom-checkbox" on:click={select_all_toggle} />
                <div class="inline-flex px-4 bg-primary">
                    {move || {source.get().len()}}
                </div>
            </div>
            <For
                each=move || source.get()
                key=|image| image.id.clone()
                children=move |new_image: DisplayImage| {
                    new_image
                }
            />
        </div>
    }
}

pub fn add_image(new_image: DisplayImage) {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    app_state.input_files.update(|images| images.push(new_image));
}

fn process_files(file_list: FileList) {
    let files: Vec<File> = (0..file_list.length())
        .filter_map(|i| file_list.get(i))
        .collect();
    let reusable_buffer = std::rc::Rc::new(RefCell::new(Vec::with_capacity(8192)));

    for file in files {
        let file_reader = web_sys::FileReader::new().unwrap();
        let file_reader = std::rc::Rc::new(file_reader);
        let file_reader_clone = file_reader.clone();

        let file_name = file.name();
        let buffer_clone = reusable_buffer.clone();

        let onload = Closure::wrap(Box::new(move |_: Event| {
            if let Ok(buffer) = file_reader_clone.result() {
                let uint8_array = js_sys::Uint8Array::new(&buffer);

                let vec = uint8_array.to_vec();
                let format = image::guess_format(&vec).ok().expect("Invalid file format!");

                // Create DynamicImage from memory
                if let Ok(img) = image::load_from_memory(&vec) {
                    let mut buffer = buffer_clone.borrow_mut();
                    buffer.clear();  // Clear the buffer before reuse
                    add_image(DisplayImage {
                            id: generate_unique_key(),
                            is_completed: false,
                            is_selected: create_rw_signal(false),
                            name: file_name.clone(),
                            in_filetype: format.extensions_str()[0],
                            out_filetype: None,
                            time_completed: None,
                            preview: generate_sample_image(&img, &mut buffer),
                            image: img,
                        result: vec![],
                        in_file: Default::default(),
                        out_file: None,
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);

        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        file_reader.read_as_array_buffer(&file).unwrap();
        onload.forget();
    }
}
