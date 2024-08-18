use std::rc::Rc;
use std::sync::mpsc;
use leptos::{create_rw_signal, SignalUpdate};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use crate::AppState;

#[wasm_bindgen(module = "/static/script.js")]
extern "C" {
    pub fn downloadFile(filename: &str, data: js_sys::Uint8Array);
}


// #[wasm_bindgen]
// pub fn run_app() -> Result<(), JsValue> {
//     let (tx, rx) = mpsc::channel();
//
//     let state = Rc::new(AppState {
//         input_files: create_rw_signal(Vec::new()),
//         queued_files: create_rw_signal(Vec::new()),
//         output_files: create_rw_signal(Vec::new()),
//     });
//
//
//     let task_state = state.clone();
//     // Spawn the processor task
//     spawn_local(task_state.step_queue(rx));
//
//     // Example of how to add a file to the queue
//     let tx_clone = tx.clone();
//     let app_state_clone = state.clone();
//     spawn_local(async move {
//         loop {
//
//             app_state_clone.queued_files.update(|queued| {
//                 // queued.push(FileItem { /* ... */ });
//             });
//
//             tx_clone.clone().send(()).expect("Failed to send notification");
//         }
//     });
//
//     Ok(())
// }
