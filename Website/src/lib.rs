#![feature(associated_type_defaults)]
pub mod components;
pub mod pages;
pub mod shared;
#[cfg(feature = "ssr")]
pub mod db;
#[cfg(feature = "ssr")]
pub mod domain;
pub mod auth;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use crate::pages::main::App;
	use std::panic;
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	_ = console_log::init_with_level(log::Level::Error);

	leptos::mount_to_body(App);
}