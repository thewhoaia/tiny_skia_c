#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};
use tiny_skia::{Color, FillRule, Paint, Path, PathBuilder, Point, Rect, Transform};

/// @brief List of possible errors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_point {
    x: f32,
    y: f32,
}

impl From<Point> for ts_point {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_transform {
    sx: f32,
    kx: f32,
    ky: f32,
    sy: f32,
    tx: f32,
    ty: f32,
}

impl From<Transform> for ts_transform {
    fn from(value: Transform) -> Self {
        Self {
            sx: value.sx,
            kx: value.kx,
            ky: value.ky,
            sy: value.sy,
            tx: value.tx,
            ty: value.ty,
        }
    }
}

impl From<ts_transform> for Transform {
    fn from(value: ts_transform) -> Self {
        Self {
            sx: value.sx,
            kx: value.kx,
            ky: value.ky,
            sy: value.sy,
            tx: value.tx,
            ty: value.ty,
        }
    }
}

pub struct ts_path_builder(PathBuilder);
pub struct ts_path(Path);

#[repr(C)]
#[derive(Debug)]
pub struct ts_color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<ts_color> for Color {
    fn from(value: ts_color) -> Self {
        Self::from_rgba8(value.r, value.g, value.b, value.a)
    }
}

pub struct ts_pixmap(tiny_skia::Pixmap);

#[repr(C)]
pub struct ts_rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl From<ts_rect> for Rect {
    fn from(value: ts_rect) -> Self {
        Rect::from_ltrb(value.x0, value.y0, value.x1, value.y1).unwrap()
    }
}

#[no_mangle]
pub extern "C" fn ts_transform_identity() -> ts_transform {
    ts_transform {
        sx: 1.0,
        kx: 0.0,
        ky: 0.0,
        sy: 1.0,
        tx: 0.0,
        ty: 0.0,
    }
}

#[no_mangle]
pub extern "C" fn ts_transform_scale(sx: f32, sy: f32) -> ts_transform {
    ts_transform {
        sx,
        kx: 0.0,
        ky: 0.0,
        sy,
        tx: 0.0,
        ty: 0.0,
    }
}

#[no_mangle]
pub extern "C" fn ts_transform_combine(a: ts_transform, b: ts_transform) -> ts_transform {
    let a: Transform = a.into();
    let b: Transform = b.into();
    a.pre_concat(b).into()
}

#[no_mangle]
pub unsafe extern "C" fn ts_path_builder_create() -> *mut ts_path_builder {
    Box::into_raw(Box::new(ts_path_builder(PathBuilder::new())))
}

#[no_mangle]
pub unsafe extern "C" fn ts_move_to(p: *mut ts_path_builder, x: f32, y: f32) {
    (*p).0.move_to(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn ts_line_to(p: *mut ts_path_builder, x: f32, y: f32) {
    (*p).0.line_to(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn ts_close(p: *mut ts_path_builder) {
    (*p).0.close();
}

#[no_mangle]
pub unsafe extern "C" fn ts_path_builder_finish(b: *mut ts_path_builder) -> *mut ts_path {
    let path_builder = Box::from_raw(b);
    Box::into_raw(Box::new(ts_path(path_builder.0.finish().unwrap())))
}

#[no_mangle]
pub unsafe extern "C" fn ts_path_len(b: *mut ts_path) -> usize {
    (*b).0.len()
}

#[no_mangle]
pub unsafe extern "C" fn ts_path_destroy(b: *mut ts_path) {
    let _ = Box::from_raw(b);
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_create(width: u32, height: u32) -> *mut ts_pixmap {
    let pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    Box::into_raw(Box::new(ts_pixmap(pixmap)))
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_destroy(pixmap: *mut ts_pixmap) {
    let _ = Box::from_raw(pixmap);
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_save(pixmap: *mut ts_pixmap, path: *const c_char) {
    let path = CStr::from_ptr(path).to_str().unwrap();
    (*pixmap).0.save_png(path).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_fill_path(
    pixmap: *mut ts_pixmap,
    path: *const ts_path,
    transform: ts_transform,
    color: ts_color,
) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);

    (*pixmap).0.fill_path(
        &(*path).0,
        &paint,
        FillRule::Winding,
        transform.into(),
        None
    );
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_fill_rect(
    pixmap: *mut ts_pixmap,
    rect: ts_rect,
    transform: ts_transform,
    color: ts_color,
) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);

    println!("{:?}", color);

    (*pixmap).0.fill_rect(
        rect.into(),
        &paint,
        transform.into(),
        None
    );
}

pub struct ts_argb(Vec<u8>);

#[no_mangle]
pub unsafe extern "C" fn ts_data(pixmap: *const ts_pixmap) -> *mut ts_argb {
    let mut buffer = Vec::with_capacity((*pixmap).0.data().len());

    for pixel in (*pixmap).0.pixels() {
        // let pixel = pixel.demultiply();
        // buffer.push(data[3]);
        buffer.extend_from_slice(&[pixel.blue(), pixel.green(), pixel.red(), pixel.alpha()]);
    }

    Box::into_raw(Box::new(ts_argb(buffer)))
}

#[no_mangle]
pub unsafe extern "C" fn ts_argb_data(data: *const ts_argb) -> *const u8 {
    (*data).0.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn ts_argb_destroy(data: *mut ts_argb) {
    let _ = Box::from_raw(data);
}
