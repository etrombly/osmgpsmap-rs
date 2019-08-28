// This file was generated by gir (https://github.com/gtk-rs/gir @ c0f523f+)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use MapLayer;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use osm_gps_map_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct MapOsd(Object<osm_gps_map_sys::OsmGpsMapOsd, osm_gps_map_sys::OsmGpsMapOsdClass, MapOsdClass>) @implements MapLayer;

    match fn {
        get_type => || osm_gps_map_sys::osm_gps_map_osd_get_type(),
    }
}

impl MapOsd {
    pub fn new() -> MapOsd {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(osm_gps_map_sys::osm_gps_map_osd_new())
        }
    }
}

impl Default for MapOsd {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MAP_OSD: Option<&MapOsd> = None;

pub trait MapOsdExt: 'static {
    fn get_property_dpad_radius(&self) -> u32;

    fn set_property_dpad_radius(&self, dpad_radius: u32);

    fn get_property_osd_x(&self) -> i32;

    fn set_property_osd_x(&self, osd_x: i32);

    fn get_property_osd_y(&self) -> i32;

    fn set_property_osd_y(&self, osd_y: i32);

    fn get_property_show_coordinates(&self) -> bool;

    fn set_property_show_coordinates(&self, show_coordinates: bool);

    fn get_property_show_copyright(&self) -> bool;

    fn set_property_show_copyright(&self, show_copyright: bool);

    fn get_property_show_crosshair(&self) -> bool;

    fn set_property_show_crosshair(&self, show_crosshair: bool);

    fn get_property_show_dpad(&self) -> bool;

    fn set_property_show_dpad(&self, show_dpad: bool);

    fn get_property_show_gps_in_dpad(&self) -> bool;

    fn set_property_show_gps_in_dpad(&self, show_gps_in_dpad: bool);

    fn get_property_show_gps_in_zoom(&self) -> bool;

    fn set_property_show_gps_in_zoom(&self, show_gps_in_zoom: bool);

    fn get_property_show_scale(&self) -> bool;

    fn set_property_show_scale(&self, show_scale: bool);

    fn get_property_show_zoom(&self) -> bool;

    fn set_property_show_zoom(&self, show_zoom: bool);

    fn connect_property_dpad_radius_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_osd_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_osd_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_coordinates_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_crosshair_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_dpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_gps_in_dpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_gps_in_zoom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_zoom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MapOsd>> MapOsdExt for O {
    fn get_property_dpad_radius(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"dpad-radius\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_dpad_radius(&self, dpad_radius: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"dpad-radius\0".as_ptr() as *const _, Value::from(&dpad_radius).to_glib_none().0);
        }
    }

    fn get_property_osd_x(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"osd-x\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_osd_x(&self, osd_x: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"osd-x\0".as_ptr() as *const _, Value::from(&osd_x).to_glib_none().0);
        }
    }

    fn get_property_osd_y(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"osd-y\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_osd_y(&self, osd_y: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"osd-y\0".as_ptr() as *const _, Value::from(&osd_y).to_glib_none().0);
        }
    }

    fn get_property_show_coordinates(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-coordinates\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_coordinates(&self, show_coordinates: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-coordinates\0".as_ptr() as *const _, Value::from(&show_coordinates).to_glib_none().0);
        }
    }

    fn get_property_show_copyright(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-copyright\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_copyright(&self, show_copyright: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-copyright\0".as_ptr() as *const _, Value::from(&show_copyright).to_glib_none().0);
        }
    }

    fn get_property_show_crosshair(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-crosshair\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_crosshair(&self, show_crosshair: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-crosshair\0".as_ptr() as *const _, Value::from(&show_crosshair).to_glib_none().0);
        }
    }

    fn get_property_show_dpad(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-dpad\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_dpad(&self, show_dpad: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-dpad\0".as_ptr() as *const _, Value::from(&show_dpad).to_glib_none().0);
        }
    }

    fn get_property_show_gps_in_dpad(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-gps-in-dpad\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_gps_in_dpad(&self, show_gps_in_dpad: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-gps-in-dpad\0".as_ptr() as *const _, Value::from(&show_gps_in_dpad).to_glib_none().0);
        }
    }

    fn get_property_show_gps_in_zoom(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-gps-in-zoom\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_gps_in_zoom(&self, show_gps_in_zoom: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-gps-in-zoom\0".as_ptr() as *const _, Value::from(&show_gps_in_zoom).to_glib_none().0);
        }
    }

    fn get_property_show_scale(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-scale\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_scale(&self, show_scale: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-scale\0".as_ptr() as *const _, Value::from(&show_scale).to_glib_none().0);
        }
    }

    fn get_property_show_zoom(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-zoom\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_zoom(&self, show_zoom: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-zoom\0".as_ptr() as *const _, Value::from(&show_zoom).to_glib_none().0);
        }
    }

    fn connect_property_dpad_radius_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dpad_radius_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dpad-radius\0".as_ptr() as *const _,
                Some(transmute(notify_dpad_radius_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_osd_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_osd_x_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::osd-x\0".as_ptr() as *const _,
                Some(transmute(notify_osd_x_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_osd_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_osd_y_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::osd-y\0".as_ptr() as *const _,
                Some(transmute(notify_osd_y_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_coordinates_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_coordinates_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-coordinates\0".as_ptr() as *const _,
                Some(transmute(notify_show_coordinates_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_copyright_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-copyright\0".as_ptr() as *const _,
                Some(transmute(notify_show_copyright_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_crosshair_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_crosshair_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-crosshair\0".as_ptr() as *const _,
                Some(transmute(notify_show_crosshair_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_dpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_dpad_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-dpad\0".as_ptr() as *const _,
                Some(transmute(notify_show_dpad_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_gps_in_dpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_gps_in_dpad_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-gps-in-dpad\0".as_ptr() as *const _,
                Some(transmute(notify_show_gps_in_dpad_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_gps_in_zoom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_gps_in_zoom_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-gps-in-zoom\0".as_ptr() as *const _,
                Some(transmute(notify_show_gps_in_zoom_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_scale_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-scale\0".as_ptr() as *const _,
                Some(transmute(notify_show_scale_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_zoom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_zoom_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-zoom\0".as_ptr() as *const _,
                Some(transmute(notify_show_zoom_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MapOsd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MapOsd")
    }
}
