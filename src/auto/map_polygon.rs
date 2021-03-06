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
use MapTrack;

glib_wrapper! {
    pub struct MapPolygon(Object<osm_gps_map_sys::OsmGpsMapPolygon, osm_gps_map_sys::OsmGpsMapPolygonClass, MapPolygonClass>);

    match fn {
        get_type => || osm_gps_map_sys::osm_gps_map_polygon_get_type(),
    }
}

impl MapPolygon {
    pub fn new() -> MapPolygon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(osm_gps_map_sys::osm_gps_map_polygon_new())
        }
    }
}

impl Default for MapPolygon {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct MapPolygonBuilder {
    breakable: Option<bool>,
    editable: Option<bool>,
    shade_alpha: Option<f32>,
    shaded: Option<bool>,
    //track: /*Unknown type*/,
    visible: Option<bool>,
}

impl MapPolygonBuilder {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn build(self) -> MapPolygon {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref breakable) = self.breakable {
            properties.push(("breakable", breakable));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref shade_alpha) = self.shade_alpha {
            properties.push(("shade-alpha", shade_alpha));
        }
        if let Some(ref shaded) = self.shaded {
            properties.push(("shaded", shaded));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        let ret = glib::Object::new(MapPolygon::static_type(), &properties)
            .expect("object new")
            .downcast::<MapPolygon>()
            .expect("downcast");
    ret
    }

    pub fn breakable(mut self, breakable: bool) -> Self {
        self.breakable = Some(breakable);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn shade_alpha(mut self, shade_alpha: f32) -> Self {
        self.shade_alpha = Some(shade_alpha);
        self
    }

    pub fn shaded(mut self, shaded: bool) -> Self {
        self.shaded = Some(shaded);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }
}

pub const NONE_MAP_POLYGON: Option<&MapPolygon> = None;

pub trait MapPolygonExt: 'static {
    fn get_track(&self) -> Option<MapTrack>;

    fn get_property_breakable(&self) -> bool;

    fn set_property_breakable(&self, breakable: bool);

    fn get_property_editable(&self) -> bool;

    fn set_property_editable(&self, editable: bool);

    fn get_property_shade_alpha(&self) -> f32;

    fn set_property_shade_alpha(&self, shade_alpha: f32);

    fn get_property_shaded(&self) -> bool;

    fn set_property_shaded(&self, shaded: bool);

    //fn set_property_track(&self, track: /*Unimplemented*/Fundamental: Pointer);

    fn get_property_visible(&self) -> bool;

    fn set_property_visible(&self, visible: bool);

    fn connect_property_breakable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shade_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shaded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MapPolygon>> MapPolygonExt for O {
    fn get_track(&self) -> Option<MapTrack> {
        unsafe {
            from_glib_none(osm_gps_map_sys::osm_gps_map_polygon_get_track(self.as_ref().to_glib_none().0))
        }
    }

    fn get_property_breakable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"breakable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `breakable` getter").unwrap()
        }
    }

    fn set_property_breakable(&self, breakable: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"breakable\0".as_ptr() as *const _, Value::from(&breakable).to_glib_none().0);
        }
    }

    fn get_property_editable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"editable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `editable` getter").unwrap()
        }
    }

    fn set_property_editable(&self, editable: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"editable\0".as_ptr() as *const _, Value::from(&editable).to_glib_none().0);
        }
    }

    fn get_property_shade_alpha(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"shade-alpha\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `shade-alpha` getter").unwrap()
        }
    }

    fn set_property_shade_alpha(&self, shade_alpha: f32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"shade-alpha\0".as_ptr() as *const _, Value::from(&shade_alpha).to_glib_none().0);
        }
    }

    fn get_property_shaded(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"shaded\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `shaded` getter").unwrap()
        }
    }

    fn set_property_shaded(&self, shaded: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"shaded\0".as_ptr() as *const _, Value::from(&shaded).to_glib_none().0);
        }
    }

    //fn set_property_track(&self, track: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"track\0".as_ptr() as *const _, Value::from(&track).to_glib_none().0);
    //    }
    //}

    fn get_property_visible(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"visible\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `visible` getter").unwrap()
        }
    }

    fn set_property_visible(&self, visible: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"visible\0".as_ptr() as *const _, Value::from(&visible).to_glib_none().0);
        }
    }

    fn connect_property_breakable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_breakable_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapPolygon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapPolygon>
        {
            let f: &F = &*(f as *const F);
            f(&MapPolygon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::breakable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_breakable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editable_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapPolygon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapPolygon>
        {
            let f: &F = &*(f as *const F);
            f(&MapPolygon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_editable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_shade_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shade_alpha_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapPolygon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapPolygon>
        {
            let f: &F = &*(f as *const F);
            f(&MapPolygon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shade-alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_shade_alpha_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_shaded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shaded_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapPolygon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapPolygon>
        {
            let f: &F = &*(f as *const F);
            f(&MapPolygon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shaded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_shaded_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapPolygon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapPolygon>
        {
            let f: &F = &*(f as *const F);
            f(&MapPolygon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::track\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_track_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapPolygon, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapPolygon>
        {
            let f: &F = &*(f as *const F);
            f(&MapPolygon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_visible_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MapPolygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MapPolygon")
    }
}
