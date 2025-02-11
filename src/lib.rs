use tiny_skia::{Point, Transform};

/// @brief List of possible errors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_point {
    x: f32,
    y: f32
}

impl Into<Point> for ts_point {
    fn into(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
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
    ty: f32
}

impl Into<Transform> for ts_transform {
    fn into(self) -> Transform {
        Transform {
            sx: self.sx,
            kx: self.kx,
            ky: self.ky,
            sy: self.sy,
            tx: self.tx,
            ty: self.ty,
        }
    }
}

impl Into<ts_transform> for Transform {
    fn into(self) -> ts_transform {
        ts_transform {
            sx: self.sx,
            kx: self.kx,
            ky: self.ky,
            sy: self.sy,
            tx: self.tx,
            ty: self.ty,
        }
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