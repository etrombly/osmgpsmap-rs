//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

extern crate gio;
extern crate gtk;
extern crate osmgpsmap;

use gio::prelude::*;
use gtk::prelude::*;
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

    let map = Map::new();

    window.add(&map);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.basic"),
                                            gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
