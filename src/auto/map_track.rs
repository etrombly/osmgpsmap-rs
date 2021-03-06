// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
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
use libc;
use osm_gps_map_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use MapPoint;

glib_wrapper! {
    pub struct MapTrack(Object<osm_gps_map_sys::OsmGpsMapTrack, osm_gps_map_sys::OsmGpsMapTrackClass, MapTrackClass>);

    match fn {
        get_type => || osm_gps_map_sys::osm_gps_map_track_get_type(),
    }
}

impl MapTrack {
    pub fn new() -> MapTrack {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(osm_gps_map_sys::osm_gps_map_track_new())
        }
    }
}

impl Default for MapTrack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct MapTrackBuilder {
    alpha: Option<f32>,
    color: Option<gdk::RGBA>,
    editable: Option<bool>,
    line_width: Option<f32>,
    //track: /*Unknown type*/,
    visible: Option<bool>,
}

impl MapTrackBuilder {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn build(self) -> MapTrack {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref alpha) = self.alpha {
            properties.push(("alpha", alpha));
        }
        if let Some(ref color) = self.color {
            properties.push(("color", color));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref line_width) = self.line_width {
            properties.push(("line-width", line_width));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        let ret = glib::Object::new(MapTrack::static_type(), &properties)
            .expect("object new")
            .downcast::<MapTrack>()
            .expect("downcast");
    ret
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    pub fn color(mut self, color: &gdk::RGBA) -> Self {
        self.color = Some(color.clone());
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = Some(line_width);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }
}

pub const NONE_MAP_TRACK: Option<&MapTrack> = None;

pub trait MapTrackExt: 'static {
    //fn add_point(&self, point: &MapPoint);

    fn get_color(&self, color: &mut gdk::RGBA);

    fn get_length(&self) -> f64;

    fn get_point(&self, pos: i32) -> Option<MapPoint>;

    fn get_points(&self) -> Vec<MapPoint>;

    fn insert_point(&self, np: &mut MapPoint, pos: i32);

    fn n_points(&self) -> i32;

    fn remove_point(&self, pos: i32);

    fn set_color(&self, color: &mut gdk::RGBA);

    fn get_property_alpha(&self) -> f32;

    fn set_property_alpha(&self, alpha: f32);

    fn get_property_editable(&self) -> bool;

    fn set_property_editable(&self, editable: bool);

    fn get_property_line_width(&self) -> f32;

    fn set_property_line_width(&self, line_width: f32);

    //fn get_property_track(&self) -> /*Unimplemented*/Fundamental: Pointer;

    fn get_property_visible(&self) -> bool;

    fn set_property_visible(&self, visible: bool);

    fn connect_point_added<F: Fn(&Self, &MapPoint) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_point_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_point_inserted<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_point_removed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MapTrack>> MapTrackExt for O {
    //fn add_point(&self, point: &MapPoint) {
    //    unsafe { TODO: call osm_gps_map_sys:osm_gps_map_track_add_point() }
    //}

    fn get_color(&self, color: &mut gdk::RGBA) {
        unsafe {
            osm_gps_map_sys::osm_gps_map_track_get_color(self.as_ref().to_glib_none().0, color.to_glib_none_mut().0);
        }
    }

    fn get_length(&self) -> f64 {
        unsafe {
            osm_gps_map_sys::osm_gps_map_track_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn get_point(&self, pos: i32) -> Option<MapPoint> {
        unsafe {
            from_glib_full(osm_gps_map_sys::osm_gps_map_track_get_point(self.as_ref().to_glib_none().0, pos))
        }
    }

    fn get_points(&self) -> Vec<MapPoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(osm_gps_map_sys::osm_gps_map_track_get_points(self.as_ref().to_glib_none().0))
        }
    }

    fn insert_point(&self, np: &mut MapPoint, pos: i32) {
        unsafe {
            osm_gps_map_sys::osm_gps_map_track_insert_point(self.as_ref().to_glib_none().0, np.to_glib_none_mut().0, pos);
        }
    }

    fn n_points(&self) -> i32 {
        unsafe {
            osm_gps_map_sys::osm_gps_map_track_n_points(self.as_ref().to_glib_none().0)
        }
    }

    fn remove_point(&self, pos: i32) {
        unsafe {
            osm_gps_map_sys::osm_gps_map_track_remove_point(self.as_ref().to_glib_none().0, pos);
        }
    }

    fn set_color(&self, color: &mut gdk::RGBA) {
        unsafe {
            osm_gps_map_sys::osm_gps_map_track_set_color(self.as_ref().to_glib_none().0, color.to_glib_none_mut().0);
        }
    }

    fn get_property_alpha(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"alpha\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `alpha` getter").unwrap()
        }
    }

    fn set_property_alpha(&self, alpha: f32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"alpha\0".as_ptr() as *const _, Value::from(&alpha).to_glib_none().0);
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

    fn get_property_line_width(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"line-width\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `line-width` getter").unwrap()
        }
    }

    fn set_property_line_width(&self, line_width: f32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"line-width\0".as_ptr() as *const _, Value::from(&line_width).to_glib_none().0);
        }
    }

    //fn get_property_track(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"track\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `track` getter").unwrap()
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

    fn connect_point_added<F: Fn(&Self, &MapPoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn point_added_trampoline<P, F: Fn(&P, &MapPoint) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, arg1: *mut osm_gps_map_sys::OsmGpsMapPoint, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(arg1))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"point-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(point_added_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_point_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn point_changed_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, object: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref(), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"point-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(point_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_point_inserted<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn point_inserted_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, object: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref(), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"point-inserted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(point_inserted_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_point_removed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn point_removed_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, object: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref(), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"point-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(point_removed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alpha_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_alpha_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_color_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editable_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_editable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_line_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_line_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::line-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_line_width_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P, F: Fn(&P) + 'static>(this: *mut osm_gps_map_sys::OsmGpsMapTrack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<MapTrack>
        {
            let f: &F = &*(f as *const F);
            f(&MapTrack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_visible_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MapTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MapTrack")
    }
}
