extern crate osm_gps_map_sys as ffi;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate cairo;
extern crate libc;

mod auto;

pub use auto::*;
