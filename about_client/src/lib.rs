#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasm_logger;
extern crate web_sys;
extern crate ybc;
extern crate yew;
extern crate yew_router;

mod app;

use app::App;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "console_errof_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook:set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc:WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn run() {
    set_panic_hook();

    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Hello WASM");
    yew::start_app::<App>();
}
