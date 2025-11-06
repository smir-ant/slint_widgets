slint::include_modules!();

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    // Redirect panic messages to the console.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Main::new().unwrap().run().unwrap();
}
