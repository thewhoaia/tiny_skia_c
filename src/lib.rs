#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};
use tiny_skia::{
    BlendMode, Color, FillRule, Paint, Path, PathBuilder, Point, Rect, Stroke, Transform,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_point {
    x: f32,
    y: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ts_blend_mode {
    SourceOver,
    SourceCopy,
}

impl From<ts_blend_mode> for BlendMode {
    fn from(value: ts_blend_mode) -> Self {
        match value {
            ts_blend_mode::SourceOver => BlendMode::SourceOver,
            ts_blend_mode::SourceCopy => BlendMode::Source,
        }
    }
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

#[repr(C)]
pub enum ts_fill_rule {
    Winding,
    EvenOdd
}

impl From<ts_fill_rule> for FillRule {
    fn from(value: ts_fill_rule) -> Self {
        match value {
            ts_fill_rule::Winding => FillRule::Winding,
            ts_fill_rule::EvenOdd => FillRule::EvenOdd
        }
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
pub extern "C" fn ts_transform_translate(tx: f32, ty: f32) -> ts_transform {
    ts_transform {
        sx: 1.0,
        kx: 0.0,
        ky: 0.0,
        sy: 1.0,
        tx,
        ty,
    }
}

#[no_mangle]
pub extern "C" fn ts_transform_rotate(angle: f32) -> ts_transform {
    Transform::from_rotate(angle).into()
}

#[no_mangle]
pub extern "C" fn ts_transform_rotate_at(angle: f32, cx: f32, cy: f32) -> ts_transform {
    Transform::from_rotate_at(angle, cx, cy).into()
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
pub unsafe extern "C" fn ts_quad_to(p: *mut ts_path_builder, x0: f32, y0: f32, x1: f32, y1: f32) {
    (*p).0.quad_to(x0, y0, x1, y1);
}

#[no_mangle]
pub unsafe extern "C" fn ts_cubic_to(p: *mut ts_path_builder, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
    (*p).0.cubic_to(x0, y0, x1, y1, x2, y2);
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
pub unsafe extern "C" fn ts_rounded_rect(rect: ts_rect, mut rx: f32, mut ry: f32) -> *mut ts_path {
    let (x, y, width, height) = (rect.x0, rect.y0, rect.x1 - rect.x0, rect.y1- rect.y0);

    rx = rx.min(width / 2.0);
    ry = ry.min(height / 2.0);

    let mut builder = PathBuilder::new();
    builder.move_to(x + rx, y);

    builder.line_to(x + width - rx, y);
    builder.arc_to(rx, ry, 0.0, false, true, x + width, y + ry);

    builder.line_to(x + width, y + height - ry);
    builder.arc_to(rx, ry, 0.0, false, true, x + width - rx, y + height);

    builder.line_to(x + rx, y + height);
    builder.arc_to(rx, ry, 0.0, false, true, x, y + height - ry);

    builder.line_to(x, y + ry);
    builder.arc_to(rx, ry, 0.0, false, true, x + rx, y);

    builder.close();

    Box::into_raw(Box::new(ts_path(builder.finish().unwrap())))
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
    fill_rule: ts_fill_rule,
    blend_mode: ts_blend_mode,
) {
    let mut paint = Paint {
        blend_mode: blend_mode.into(),
        ..Default::default()
    };
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);

    (*pixmap).0.fill_path(
        &(*path).0,
        &paint,
        fill_rule.into(),
        transform.into(),
        None,
    );
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_fill_rect(
    pixmap: *mut ts_pixmap,
    rect: ts_rect,
    transform: ts_transform,
    color: ts_color,
    blend_mode: ts_blend_mode,
) {
    let mut paint = Paint {
        blend_mode: blend_mode.into(),
        ..Default::default()
    };
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);

    (*pixmap)
        .0
        .fill_rect(rect.into(), &paint, transform.into(), None);
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

#[repr(C)]
pub struct ts_stroke {
    width: f32
}

impl From<ts_stroke> for Stroke {
    fn from(value: ts_stroke) -> Self {
        Self {
            width: value.width,
            ..Default::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_stroke_path(
    pixmap: *mut ts_pixmap,
    path: *const ts_path,
    transform: ts_transform,
    color: ts_color,
    stroke: ts_stroke,
    blend_mode: ts_blend_mode,
) {
    let mut paint = Paint {
        blend_mode: blend_mode.into(),
        ..Default::default()
    };
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);

    (*pixmap).0.stroke_path(
        &(*path).0,
        &paint,
        &stroke.into(),
        transform.into(),
        None,
    );
}

#[no_mangle]
pub unsafe extern "C" fn ts_pixmap_stroke_rect(
    pixmap: *mut ts_pixmap,
    rect: ts_rect,
    transform: ts_transform,
    color: ts_color,
    stroke: ts_stroke,
    blend_mode: ts_blend_mode,
) {
    let mut paint = Paint {
        blend_mode: blend_mode.into(),
        ..Default::default()
    };
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);

    (*pixmap)
        .0
        .stroke_path(
            &PathBuilder::from_rect(rect.into()),
            &paint,
            &stroke.into(),
            transform.into(),
            None,
        );
}

trait PathBuilderExt {
    fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32,
        y: f32,
    );
}

impl PathBuilderExt for PathBuilder {
    fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32,
        y: f32,
    ) {
        let prev = match self.last_point() {
            Some(v) => v,
            None => return,
        };

        let svg_arc = kurbo::SvgArc {
            from: kurbo::Point::new(prev.x as f64, prev.y as f64),
            to: kurbo::Point::new(x as f64, y as f64),
            radii: kurbo::Vec2::new(rx as f64, ry as f64),
            x_rotation: (x_axis_rotation as f64).to_radians(),
            large_arc,
            sweep,
        };

        match kurbo::Arc::from_svg_arc(&svg_arc) {
            Some(arc) => {
                arc.to_cubic_beziers(0.1, |p1, p2, p| {
                    self.cubic_to(
                        p1.x as f32,
                        p1.y as f32,
                        p2.x as f32,
                        p2.y as f32,
                        p.x as f32,
                        p.y as f32,
                    );
                });
            }
            None => {
                self.line_to(x, y);
            }
        }
    }
}
