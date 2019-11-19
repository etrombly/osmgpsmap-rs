// This file was generated by gir (https://github.com/gtk-rs/gir @ fb0b31c+)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal)]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gtk_sys as gtk;
extern crate cairo_sys as cairo;
extern crate gdk_sys as gdk;
extern crate gdk_pixbuf_sys as gdk_pixbuf;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type OsmGpsMapKey_t = c_int;
pub const OSM_GPS_MAP_KEY_FULLSCREEN: OsmGpsMapKey_t = 0;
pub const OSM_GPS_MAP_KEY_ZOOMIN: OsmGpsMapKey_t = 1;
pub const OSM_GPS_MAP_KEY_ZOOMOUT: OsmGpsMapKey_t = 2;
pub const OSM_GPS_MAP_KEY_UP: OsmGpsMapKey_t = 3;
pub const OSM_GPS_MAP_KEY_DOWN: OsmGpsMapKey_t = 4;
pub const OSM_GPS_MAP_KEY_LEFT: OsmGpsMapKey_t = 5;
pub const OSM_GPS_MAP_KEY_RIGHT: OsmGpsMapKey_t = 6;
pub const OSM_GPS_MAP_KEY_MAX: OsmGpsMapKey_t = 7;

pub type OsmGpsMapSource_t = c_int;
pub const OSM_GPS_MAP_SOURCE_NULL: OsmGpsMapSource_t = 0;
pub const OSM_GPS_MAP_SOURCE_OPENSTREETMAP: OsmGpsMapSource_t = 1;
pub const OSM_GPS_MAP_SOURCE_OPENSTREETMAP_RENDERER: OsmGpsMapSource_t = 2;
pub const OSM_GPS_MAP_SOURCE_OPENAERIALMAP: OsmGpsMapSource_t = 3;
pub const OSM_GPS_MAP_SOURCE_MAPS_FOR_FREE: OsmGpsMapSource_t = 4;
pub const OSM_GPS_MAP_SOURCE_OPENCYCLEMAP: OsmGpsMapSource_t = 5;
pub const OSM_GPS_MAP_SOURCE_OSM_PUBLIC_TRANSPORT: OsmGpsMapSource_t = 6;
pub const OSM_GPS_MAP_SOURCE_GOOGLE_STREET: OsmGpsMapSource_t = 7;
pub const OSM_GPS_MAP_SOURCE_GOOGLE_SATELLITE: OsmGpsMapSource_t = 8;
pub const OSM_GPS_MAP_SOURCE_GOOGLE_HYBRID: OsmGpsMapSource_t = 9;
pub const OSM_GPS_MAP_SOURCE_VIRTUAL_EARTH_STREET: OsmGpsMapSource_t = 10;
pub const OSM_GPS_MAP_SOURCE_VIRTUAL_EARTH_SATELLITE: OsmGpsMapSource_t = 11;
pub const OSM_GPS_MAP_SOURCE_VIRTUAL_EARTH_HYBRID: OsmGpsMapSource_t = 12;
pub const OSM_GPS_MAP_SOURCE_OSMC_TRAILS: OsmGpsMapSource_t = 13;
pub const OSM_GPS_MAP_SOURCE_LAST: OsmGpsMapSource_t = 14;

// Constants
pub const OSM_GPS_MAP_CACHE_AUTO: *const c_char = b"auto://\0" as *const u8 as *const c_char;
pub const OSM_GPS_MAP_CACHE_DISABLED: *const c_char = b"none://\0" as *const u8 as *const c_char;
pub const OSM_GPS_MAP_CACHE_FRIENDLY: *const c_char = b"friendly://\0" as *const u8 as *const c_char;
pub const OSM_GPS_MAP_INVALID: c_int = 0;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapClass {
    pub parent_class: gtk::GtkDrawingAreaClass,
    pub draw_gps_point: Option<unsafe extern "C" fn(*mut OsmGpsMap, *mut cairo::cairo_t)>,
}

impl ::std::fmt::Debug for OsmGpsMapClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("draw_gps_point", &self.draw_gps_point)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapImageClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for OsmGpsMapImageClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapImageClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _OsmGpsMapImagePrivate(c_void);

pub type OsmGpsMapImagePrivate = *mut _OsmGpsMapImagePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapLayerIface {
    pub parent: gobject::GTypeInterface,
    pub render: Option<unsafe extern "C" fn(*mut OsmGpsMapLayer, *mut OsmGpsMap)>,
    pub draw: Option<unsafe extern "C" fn(*mut OsmGpsMapLayer, *mut OsmGpsMap, *mut cairo::cairo_t)>,
    pub busy: Option<unsafe extern "C" fn(*mut OsmGpsMapLayer) -> gboolean>,
    pub button_press: Option<unsafe extern "C" fn(*mut OsmGpsMapLayer, *mut OsmGpsMap, *mut gdk::GdkEventButton) -> gboolean>,
}

impl ::std::fmt::Debug for OsmGpsMapLayerIface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapLayerIface @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("render", &self.render)
         .field("draw", &self.draw)
         .field("busy", &self.busy)
         .field("button_press", &self.button_press)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapOsdClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for OsmGpsMapOsdClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapOsdClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _OsmGpsMapOsdPrivate(c_void);

pub type OsmGpsMapOsdPrivate = *mut _OsmGpsMapOsdPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapPoint {
    pub rlat: c_float,
    pub rlon: c_float,
}

impl ::std::fmt::Debug for OsmGpsMapPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapPoint @ {:?}", self as *const _))
         .field("rlat", &self.rlat)
         .field("rlon", &self.rlon)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapPolygonClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for OsmGpsMapPolygonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapPolygonClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _OsmGpsMapPolygonPrivate(c_void);

pub type OsmGpsMapPolygonPrivate = *mut _OsmGpsMapPolygonPrivate;

#[repr(C)]
pub struct _OsmGpsMapPrivate(c_void);

pub type OsmGpsMapPrivate = *mut _OsmGpsMapPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapTrackClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for OsmGpsMapTrackClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapTrackClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _OsmGpsMapTrackPrivate(c_void);

pub type OsmGpsMapTrackPrivate = *mut _OsmGpsMapTrackPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMap {
    pub parent_instance: gtk::GtkDrawingArea,
    pub priv_: *mut OsmGpsMapPrivate,
}

impl ::std::fmt::Debug for OsmGpsMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMap @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapImage {
    pub parent: gobject::GObject,
    pub priv_: *mut OsmGpsMapImagePrivate,
}

impl ::std::fmt::Debug for OsmGpsMapImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapImage @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapOsd {
    pub parent: gobject::GObject,
    pub priv_: *mut OsmGpsMapOsdPrivate,
}

impl ::std::fmt::Debug for OsmGpsMapOsd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapOsd @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapPolygon {
    pub parent: gobject::GObject,
    pub priv_: *mut OsmGpsMapPolygonPrivate,
}

impl ::std::fmt::Debug for OsmGpsMapPolygon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapPolygon @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsmGpsMapTrack {
    pub parent: gobject::GObject,
    pub priv_: *mut OsmGpsMapTrackPrivate,
}

impl ::std::fmt::Debug for OsmGpsMapTrack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("OsmGpsMapTrack @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct OsmGpsMapLayer(c_void);

impl ::std::fmt::Debug for OsmGpsMapLayer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "OsmGpsMapLayer @ {:?}", self as *const _)
    }
}


extern "C" {

    //=========================================================================
    // OsmGpsMapPoint
    //=========================================================================
    pub fn osm_gps_map_point_get_type() -> GType;
    pub fn osm_gps_map_point_new_degrees(lat: c_float, lon: c_float) -> *mut OsmGpsMapPoint;
    pub fn osm_gps_map_point_new_radians(rlat: c_float, rlon: c_float) -> *mut OsmGpsMapPoint;
    pub fn osm_gps_map_point_copy(point: *const OsmGpsMapPoint) -> *mut OsmGpsMapPoint;
    pub fn osm_gps_map_point_free(point: *mut OsmGpsMapPoint);
    pub fn osm_gps_map_point_get_degrees(point: *mut OsmGpsMapPoint, lat: *mut c_float, lon: *mut c_float);
    pub fn osm_gps_map_point_get_radians(point: *mut OsmGpsMapPoint, rlat: *mut c_float, rlon: *mut c_float);
    pub fn osm_gps_map_point_set_degrees(point: *mut OsmGpsMapPoint, lat: c_float, lon: c_float);
    pub fn osm_gps_map_point_set_radians(point: *mut OsmGpsMapPoint, rlat: c_float, rlon: c_float);

    //=========================================================================
    // OsmGpsMap
    //=========================================================================
    pub fn osm_gps_map_get_type() -> GType;
    pub fn osm_gps_map_new() -> *mut gtk::GtkWidget;
    pub fn osm_gps_map_get_default_cache_directory() -> *mut c_char;
    pub fn osm_gps_map_source_get_copyright(source: OsmGpsMapSource_t) -> *const c_char;
    pub fn osm_gps_map_source_get_friendly_name(source: OsmGpsMapSource_t) -> *const c_char;
    pub fn osm_gps_map_source_get_image_format(source: OsmGpsMapSource_t) -> *const c_char;
    pub fn osm_gps_map_source_get_max_zoom(source: OsmGpsMapSource_t) -> c_int;
    pub fn osm_gps_map_source_get_min_zoom(source: OsmGpsMapSource_t) -> c_int;
    pub fn osm_gps_map_source_get_repo_uri(source: OsmGpsMapSource_t) -> *const c_char;
    pub fn osm_gps_map_source_is_valid(source: OsmGpsMapSource_t) -> gboolean;
    pub fn osm_gps_map_convert_geographic_to_screen(map: *mut OsmGpsMap, pt: *mut OsmGpsMapPoint, pixel_x: *mut c_int, pixel_y: *mut c_int);
    pub fn osm_gps_map_convert_screen_to_geographic(map: *mut OsmGpsMap, pixel_x: c_int, pixel_y: c_int, pt: *mut OsmGpsMapPoint);
    pub fn osm_gps_map_download_cancel_all(map: *mut OsmGpsMap);
    pub fn osm_gps_map_download_maps(map: *mut OsmGpsMap, pt1: *mut OsmGpsMapPoint, pt2: *mut OsmGpsMapPoint, zoom_start: c_int, zoom_end: c_int);
    pub fn osm_gps_map_get_bbox(map: *mut OsmGpsMap, pt1: *mut OsmGpsMapPoint, pt2: *mut OsmGpsMapPoint);
    pub fn osm_gps_map_get_event_location(map: *mut OsmGpsMap, event: *mut gdk::GdkEventButton) -> *mut OsmGpsMapPoint;
    pub fn osm_gps_map_get_scale(map: *mut OsmGpsMap) -> c_float;
    pub fn osm_gps_map_gps_add(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float, heading: c_float);
    pub fn osm_gps_map_gps_clear(map: *mut OsmGpsMap);
    pub fn osm_gps_map_gps_get_track(map: *mut OsmGpsMap) -> *mut OsmGpsMapTrack;
    pub fn osm_gps_map_image_add(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float, image: *mut gdk_pixbuf::GdkPixbuf) -> *mut OsmGpsMapImage;
    pub fn osm_gps_map_image_add_with_alignment(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float, image: *mut gdk_pixbuf::GdkPixbuf, xalign: c_float, yalign: c_float) -> *mut OsmGpsMapImage;
    pub fn osm_gps_map_image_add_with_alignment_z(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float, image: *mut gdk_pixbuf::GdkPixbuf, xalign: c_float, yalign: c_float, zorder: c_int) -> *mut OsmGpsMapImage;
    pub fn osm_gps_map_image_add_z(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float, image: *mut gdk_pixbuf::GdkPixbuf, zorder: c_int) -> *mut OsmGpsMapImage;
    pub fn osm_gps_map_image_remove(map: *mut OsmGpsMap, image: *mut OsmGpsMapImage) -> gboolean;
    pub fn osm_gps_map_image_remove_all(map: *mut OsmGpsMap);
    pub fn osm_gps_map_layer_add(map: *mut OsmGpsMap, layer: *mut OsmGpsMapLayer);
    pub fn osm_gps_map_layer_remove(map: *mut OsmGpsMap, layer: *mut OsmGpsMapLayer) -> gboolean;
    pub fn osm_gps_map_layer_remove_all(map: *mut OsmGpsMap);
    pub fn osm_gps_map_map_redraw(map: *mut OsmGpsMap) -> gboolean;
    pub fn osm_gps_map_map_redraw_idle(map: *mut OsmGpsMap);
    pub fn osm_gps_map_polygon_add(map: *mut OsmGpsMap, poly: *mut OsmGpsMapPolygon);
    pub fn osm_gps_map_polygon_remove(map: *mut OsmGpsMap, poly: *mut OsmGpsMapPolygon) -> gboolean;
    pub fn osm_gps_map_polygon_remove_all(map: *mut OsmGpsMap);
    pub fn osm_gps_map_scroll(map: *mut OsmGpsMap, dx: c_int, dy: c_int);
    pub fn osm_gps_map_set_center(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float);
    pub fn osm_gps_map_set_center_and_zoom(map: *mut OsmGpsMap, latitude: c_float, longitude: c_float, zoom: c_int);
    pub fn osm_gps_map_set_keyboard_shortcut(map: *mut OsmGpsMap, key: OsmGpsMapKey_t, keyval: c_uint);
    pub fn osm_gps_map_set_zoom(map: *mut OsmGpsMap, zoom: c_int) -> c_int;
    pub fn osm_gps_map_set_zoom_offset(map: *mut OsmGpsMap, zoom_offset: c_int);
    pub fn osm_gps_map_track_add(map: *mut OsmGpsMap, track: *mut OsmGpsMapTrack);
    pub fn osm_gps_map_track_remove(map: *mut OsmGpsMap, track: *mut OsmGpsMapTrack) -> gboolean;
    pub fn osm_gps_map_track_remove_all(map: *mut OsmGpsMap);
    pub fn osm_gps_map_zoom_fit_bbox(map: *mut OsmGpsMap, latitude1: c_float, latitude2: c_float, longitude1: c_float, longitude2: c_float);
    pub fn osm_gps_map_zoom_in(map: *mut OsmGpsMap) -> c_int;
    pub fn osm_gps_map_zoom_out(map: *mut OsmGpsMap) -> c_int;

    //=========================================================================
    // OsmGpsMapImage
    //=========================================================================
    pub fn osm_gps_map_image_get_type() -> GType;
    pub fn osm_gps_map_image_new() -> *mut OsmGpsMapImage;
    pub fn osm_gps_map_image_draw(object: *mut OsmGpsMapImage, cr: *mut cairo::cairo_t, rect: *mut gdk::GdkRectangle);
    pub fn osm_gps_map_image_get_point(object: *mut OsmGpsMapImage) -> *const OsmGpsMapPoint;
    pub fn osm_gps_map_image_get_rotation(object: *mut OsmGpsMapImage) -> c_float;
    pub fn osm_gps_map_image_get_zorder(object: *mut OsmGpsMapImage) -> c_int;
    pub fn osm_gps_map_image_set_rotation(object: *mut OsmGpsMapImage, rot: c_float);

    //=========================================================================
    // OsmGpsMapOsd
    //=========================================================================
    pub fn osm_gps_map_osd_get_type() -> GType;
    pub fn osm_gps_map_osd_new() -> *mut OsmGpsMapOsd;

    //=========================================================================
    // OsmGpsMapPolygon
    //=========================================================================
    pub fn osm_gps_map_polygon_get_type() -> GType;
    pub fn osm_gps_map_polygon_new() -> *mut OsmGpsMapPolygon;
    pub fn osm_gps_map_polygon_get_track(poly: *mut OsmGpsMapPolygon) -> *mut OsmGpsMapTrack;

    //=========================================================================
    // OsmGpsMapTrack
    //=========================================================================
    pub fn osm_gps_map_track_get_type() -> GType;
    pub fn osm_gps_map_track_new() -> *mut OsmGpsMapTrack;
    pub fn osm_gps_map_track_add_point(track: *mut OsmGpsMapTrack, point: *const OsmGpsMapPoint);
    pub fn osm_gps_map_track_get_color(track: *mut OsmGpsMapTrack, color: *mut gdk::GdkRGBA);
    pub fn osm_gps_map_track_get_length(track: *mut OsmGpsMapTrack) -> c_double;
    pub fn osm_gps_map_track_get_point(track: *mut OsmGpsMapTrack, pos: c_int) -> *mut OsmGpsMapPoint;
    pub fn osm_gps_map_track_get_points(track: *mut OsmGpsMapTrack) -> *mut glib::GSList;
    pub fn osm_gps_map_track_insert_point(track: *mut OsmGpsMapTrack, np: *mut OsmGpsMapPoint, pos: c_int);
    pub fn osm_gps_map_track_n_points(track: *mut OsmGpsMapTrack) -> c_int;
    pub fn osm_gps_map_track_remove_point(track: *mut OsmGpsMapTrack, pos: c_int);
    pub fn osm_gps_map_track_set_color(track: *mut OsmGpsMapTrack, color: *mut gdk::GdkRGBA);

    //=========================================================================
    // OsmGpsMapLayer
    //=========================================================================
    pub fn osm_gps_map_layer_get_type() -> GType;
    pub fn osm_gps_map_layer_busy(self_: *mut OsmGpsMapLayer) -> gboolean;
    pub fn osm_gps_map_layer_button_press(self_: *mut OsmGpsMapLayer, map: *mut OsmGpsMap, event: *mut gdk::GdkEventButton) -> gboolean;
    pub fn osm_gps_map_layer_draw(self_: *mut OsmGpsMapLayer, map: *mut OsmGpsMap, cr: *mut cairo::cairo_t);
    pub fn osm_gps_map_layer_render(self_: *mut OsmGpsMapLayer, map: *mut OsmGpsMap);

}
