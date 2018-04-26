//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

///
///    NOT CURRENTLY WORKING
///
///

extern crate gio;
extern crate gtk;
extern crate gobject_sys;
extern crate glib;
extern crate osmgpsmap;
extern crate osm_gps_map_sys as ffi;


use gio::prelude::*;
use gtk::prelude::*;
use gobject_sys::{g_object_newv, GParameter};
use glib::Value;
use glib::translate::{ToGlibPtr, from_glib_none, FromGlibPtrNone};
use osmgpsmap::Map;

use std::env::args;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("OsmGpsMap example");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let prop_name_name = "name".to_glib_none();
    let prop_name_str: Option<String> = Some("name".to_owned());
    let prop_name_value = glib::Value::from(prop_name_str.as_ref());

    let mut properties = [
        GParameter {
            name: prop_name_name.0,
            value: prop_name_value.into_raw(),
        },
    ];

    let map: Map = gtk::Widget::from_glib_none(g_object_newv(ffi::osm_gps_map_get_type(), properties.len() as u32, properties.as_mut_ptr()));

    //window.add(&map);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.basic",
                                            gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}