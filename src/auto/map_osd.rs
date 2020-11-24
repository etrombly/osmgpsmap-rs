// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use osm_gps_map_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use MapLayer;

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

#[derive(Clone, Default)]
pub struct MapOsdBuilder {
    dpad_radius: Option<u32>,
    osd_x: Option<i32>,
    osd_y: Option<i32>,
    show_coordinates: Option<bool>,
    show_copyright: Option<bool>,
    show_crosshair: Option<bool>,
    show_dpad: Option<bool>,
    show_gps_in_dpad: Option<bool>,
    show_gps_in_zoom: Option<bool>,
    show_scale: Option<bool>,
    show_zoom: Option<bool>,
}

impl MapOsdBuilder {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn build(self) -> MapOsd {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref dpad_radius) = self.dpad_radius {
            properties.push(("dpad-radius", dpad_radius));
        }
        if let Some(ref osd_x) = self.osd_x {
            properties.push(("osd-x", osd_x));
        }
        if let Some(ref osd_y) = self.osd_y {
            properties.push(("osd-y", osd_y));
        }
        if let Some(ref show_coordinates) = self.show_coordinates {
            properties.push(("show-coordinates", show_coordinates));
        }
        if let Some(ref show_copyright) = self.show_copyright {
            properties.push(("show-copyright", show_copyright));
        }
        if let Some(ref show_crosshair) = self.show_crosshair {
            properties.push(("show-crosshair", show_crosshair));
        }
        if let Some(ref show_dpad) = self.show_dpad {
            properties.push(("show-dpad", show_dpad));
        }
        if let Some(ref show_gps_in_dpad) = self.show_gps_in_dpad {
            properties.push(("show-gps-in-dpad", show_gps_in_dpad));
        }
        if let Some(ref show_gps_in_zoom) = self.show_gps_in_zoom {
            properties.push(("show-gps-in-zoom", show_gps_in_zoom));
        }
        if let Some(ref show_scale) = self.show_scale {
            properties.push(("show-scale", show_scale));
        }
        if let Some(ref show_zoom) = self.show_zoom {
            properties.push(("show-zoom", show_zoom));
        }
        let ret = glib::Object::new(MapOsd::static_type(), &properties)
            .expect("object new")
            .downcast::<MapOsd>()
            .expect("downcast");
    ret
    }

    pub fn dpad_radius(mut self, dpad_radius: u32) -> Self {
        self.dpad_radius = Some(dpad_radius);
        self
    }

    pub fn osd_x(mut self, osd_x: i32) -> Self {
        self.osd_x = Some(osd_x);
        self
    }

    pub fn osd_y(mut self, osd_y: i32) -> Self {
        self.osd_y = Some(osd_y);
        self
    }

    pub fn show_coordinates(mut self, show_coordinates: bool) -> Self {
        self.show_coordinates = Some(show_coordinates);
        self
    }

    pub fn show_copyright(mut self, show_copyright: bool) -> Self {
        self.show_copyright = Some(show_copyright);
        self
    }

    pub fn show_crosshair(mut self, show_crosshair: bool) -> Self {
        self.show_crosshair = Some(show_crosshair);
        self
    }

    pub fn show_dpad(mut self, show_dpad: bool) -> Self {
        self.show_dpad = Some(show_dpad);
        self
    }

    pub fn show_gps_in_dpad(mut self, show_gps_in_dpad: bool) -> Self {
        self.show_gps_in_dpad = Some(show_gps_in_dpad);
        self
    }

    pub fn show_gps_in_zoom(mut self, show_gps_in_zoom: bool) -> Self {
        self.show_gps_in_zoom = Some(show_gps_in_zoom);
        self
    }

    pub fn show_scale(mut self, show_scale: bool) -> Self {
        self.show_scale = Some(show_scale);
        self
    }

    pub fn show_zoom(mut self, show_zoom: bool) -> Self {
        self.show_zoom = Some(show_zoom);
        self
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
            value.get().expect("Return Value for property `dpad-radius` getter").unwrap()
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
            value.get().expect("Return Value for property `osd-x` getter").unwrap()
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
            value.get().expect("Return Value for property `osd-y` getter").unwrap()
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
            value.get().expect("Return Value for property `show-coordinates` getter").unwrap()
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
            value.get().expect("Return Value for property `show-copyright` getter").unwrap()
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
            value.get().expect("Return Value for property `show-crosshair` getter").unwrap()
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
            value.get().expect("Return Value for property `show-dpad` getter").unwrap()
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
            value.get().expect("Return Value for property `show-gps-in-dpad` getter").unwrap()
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
            value.get().expect("Return Value for property `show-gps-in-zoom` getter").unwrap()
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
            value.get().expect("Return Value for property `show-scale` getter").unwrap()
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
            value.get().expect("Return Value for property `show-zoom` getter").unwrap()
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
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dpad-radius\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_dpad_radius_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_osd_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_osd_x_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::osd-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_osd_x_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_osd_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_osd_y_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::osd-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_osd_y_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_coordinates_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_coordinates_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-coordinates\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_coordinates_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_copyright_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-copyright\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_copyright_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_crosshair_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_crosshair_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-crosshair\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_crosshair_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_dpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_dpad_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-dpad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_dpad_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_gps_in_dpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_gps_in_dpad_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-gps-in-dpad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_gps_in_dpad_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_gps_in_zoom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_gps_in_zoom_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-gps-in-zoom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_gps_in_zoom_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_scale_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-scale\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_scale_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show_zoom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_zoom_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapOsd, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapOsd>
        {
            let f: &F = &*(f as *const F);
            f(&MapOsd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-zoom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_show_zoom_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MapOsd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MapOsd")
    }
}
