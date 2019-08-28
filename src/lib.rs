#![allow(unused_imports)]

extern crate osm_gps_map_sys as ffi;
extern crate osm_gps_map_sys;
#[macro_use]
extern crate glib;
//extern crate glib_sys as glib_ffi;
extern crate glib_sys;
//extern crate gobject_sys as gobject_ffi;
extern crate gobject_sys;
extern crate gtk_sys as gtk_ffi;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate libc;
extern crate cairo;

pub use auto::*;

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

mod auto;

pub use auto::*;
