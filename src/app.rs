use lazy_static::lazy_static;
use leptos::{component, create_memo, create_rw_signal, provide_context, use_context, view, For, IntoView, RwSignal, SignalGet, SignalUpdate};
use leptos_mview::mview;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Event, File, FileList, HtmlInputElement};
use crate::{AppState, DisplayImage, ImageGallery};


lazy_static! {

}

#[component]
pub fn App() -> impl IntoView {
    let images = create_rw_signal(vec![]);
    let app_state = AppState { input_files: images, count: 0 };
    provide_context(app_state);


    let unfinished = vec![
        DisplayImage {
            id: 1,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 2,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 3,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 4,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 5,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 6,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 7,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 8,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 9,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 10,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 11,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 12,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 13,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id:14,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 15,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
        DisplayImage {
            id: 16,
            is_completed: false,
            name: "vacation-to-disney-213.jpg".to_owned(),
            in_filetype: "jpg",
            out_filetype: Some("png"),
            time_completed: Some("04:43:43 PM 17-08-2024".to_owned()),
        },
    ];

    mview! {
        // Content Sections
        div class="flex flex-1 items-center justify-center h-screen bg-gray-100" {
            div class="basis-1/2 flex flex-1 h-full align-center h-full" {
                div class="h-full w-1/4" {
                    ImageUploader;
                    ImageContainer id="uploaded-images" images={unfinished};
                }
                "Uploaded"
                ImageGallery;
            }
        }
    }
}

#[component]
pub fn ImageUploader() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");

    let on_files_change = move |ev: Event| {
        let app_state = app_state.clone(); // Clone here
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files() {
            process_files(file_list, app_state);
        }
    };

    view! {
        <div>
            <input type="file" accept="image/*" on:change=on_files_change multiple=true />
        </div>
    }
}

#[component]
pub fn ImageContainer(id: &'static str, images: Vec<DisplayImage>) -> impl IntoView {
    view! {
        <div id={id} class="h-5/6 w-full bg-gray-200 overflow-auto">
            <For
                each=move || images.clone()
                key=|image| image.id.clone() // Assuming id is a String or similar clonable type
                children=move |new_image: DisplayImage| {
                    new_image.render()
                }
            />
        </div>
    }
}

pub fn add_image(new_image: DisplayImage) {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    app_state.input_files.update(|images| images.push(new_image));
}

pub fn add_images(new_images: Vec<DisplayImage>) {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    app_state.input_files.update(|images| images.extend(new_images));
}

// #[component]
// pub fn ImageContainer(id: &'static str, images: Vec<DisplayImage>) -> impl IntoView {
//     let images_ref = &images;
//     view! {
//         <div id={id} class="h-5/6 w-full bg-gray-200 overflow-auto">
//             <For
//                 each=move || images_ref.iter().map()
//                 key=|image| image.id // Use a unique identifier for the key
//                 children=move |image| {
//                     image.render()
//                 }
//             />
//         </div>
//     }
//
// }

fn process_files(file_list: FileList, state: AppState) {
    let files: Vec<File> = (0..file_list.length())
        .filter_map(|i| file_list.get(i))
        .collect();

    for file in files {
        let file_reader = web_sys::FileReader::new().unwrap();
        let file_reader = std::rc::Rc::new(file_reader);
        let file_reader_clone = file_reader.clone();

        let file_name = file.name();

        let onload = Closure::wrap(Box::new(move |_: Event| {
            if let Ok(buffer) = file_reader_clone.result() {
                let uint8_array = js_sys::Uint8Array::new(&buffer);

                let vec = uint8_array.to_vec();
                let format = image::guess_format(&vec).ok().expect("Invalid file format!");

                // Create DynamicImage from memory
                if let Ok(_img) = image::load_from_memory(&vec) {
                    add_image(DisplayImage {
                            id: 0,
                            is_completed: false,
                            name: file_name.clone(),
                            in_filetype: format.extensions_str()[0],
                            out_filetype: None,
                            time_completed: None,
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);

        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        file_reader.read_as_array_buffer(&file).unwrap();
        onload.forget();
    }
}
