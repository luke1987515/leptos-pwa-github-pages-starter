mod app;
mod home;
mod not_found;
mod pwa_install;

use app::App;
use leptos::*;

fn main() {
    // Set panic hook to output Rust panics to browser console
    console_error_panic_hook::set_once();

    // Mount Leptos App component to document body
    mount_to_body(App);
}
