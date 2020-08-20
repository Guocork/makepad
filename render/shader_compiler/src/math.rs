use crate::util::PrettyPrintedFloat;
use makepad_microserde::*;
use std::fmt;

#[derive(Clone, Copy, Default, Debug, PartialEq, SerRon, DeRon)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Rect {
    
    pub fn contains(&self, x: f32, y: f32) -> bool {
        return x >= self.x && x <= self.x + self.w &&
        y >= self.y && y <= self.y + self.h;
    }
    pub fn intersects(&self, r: Rect) -> bool {
        !(
            r.x > self.x + self.w ||
            r.x + r.w < self.x ||
            r.y > self.y + self.h ||
            r.y + r.h < self.y
        )
    }
    
    pub fn contains_with_margin(&self, x: f32, y: f32, margin: &Option<Margin>) -> bool {
        if let Some(margin) = margin {
            return x >= self.x - margin.l && x <= self.x + self.w + margin.r &&
            y >= self.y - margin.t && y <= self.y + self.h + margin.b;
        }
        else {
            return self.contains(x, y);
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Align {
    pub fx: f32,
    pub fy: f32
}

impl Align {
    pub fn left_top() -> Align {Align {fx: 0., fy: 0.}}
    pub fn center_top() -> Align {Align {fx: 0.5, fy: 0.0}}
    pub fn right_top() -> Align {Align {fx: 1.0, fy: 0.0}}
    pub fn left_center() -> Align {Align {fx: 0.0, fy: 0.5}}
    pub fn center() -> Align {Align {fx: 0.5, fy: 0.5}}
    pub fn right_center() -> Align {Align {fx: 1.0, fy: 0.5}}
    pub fn left_bottom() -> Align {Align {fx: 0., fy: 1.0}}
    pub fn center_bottom() -> Align {Align {fx: 0.5, fy: 1.0}}
    pub fn right_bottom() -> Align {Align {fx: 1.0, fy: 1.0}}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Margin {
    pub l: f32,
    pub t: f32,
    pub r: f32,
    pub b: f32
}

impl Margin {
    pub fn zero() -> Margin {
        Margin {l: 0.0, t: 0.0, r: 0.0, b: 0.0}
    }
    
    pub fn all(v: f32) -> Margin {
        Margin {l: v, t: v, r: v, b: v}
    }
    
    pub fn left(v: f32) -> Margin {
        Margin {l: v, t: 0.0, r: 0.0, b: 0.0}
    }
    
    pub fn top(v: f32) -> Margin {
        Margin {l: 0.0, t: v, r: 0.0, b: 0.0}
    }
    
    pub fn right(v: f32) -> Margin {
        Margin {l: 0.0, t: 0.0, r: v, b: 0.0}
    }
    
    pub fn bottom(v: f32) -> Margin {
        Margin {l: 0.0, t: 0.0, r: 0.0, b: v}
    }
    
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Padding {
    pub l: f32,
    pub t: f32,
    pub r: f32,
    pub b: f32
}

impl Padding {
    pub fn zero() -> Padding {
        Padding {l: 0.0, t: 0.0, r: 0.0, b: 0.0}
    }
    pub fn all(v: f32) -> Padding {
        Padding {l: v, t: v, r: v, b: v}
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Copy, Clone, SerRon, DeRon)]
pub enum Axis {
    Horizontal,
    Vertical
}

impl Default for Axis {
    fn default() -> Self {
        Axis::Horizontal
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Mat4 {
    pub v: [f32; 16],
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Transform {
    pub orientation: Vec4,
    pub position: Vec3
}


#[derive(Clone, Copy, Default, Debug, PartialEq, SerRon, DeRon)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    
    pub fn distance(&self, other: &Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
/*
pub fn vec2(x:f32, y:f32)->Vec2{
    Vec2{x:x, y:y}
}*/

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/*
pub fn vec3(x:f32, y:f32, z:f32)->Vec3{
    Vec3{x:x, y:y, z:z}
}*/

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "vec4({}, {}, {}, {})",
            PrettyPrintedFloat(self.x),
            PrettyPrintedFloat(self.y),
            PrettyPrintedFloat(self.z),
            PrettyPrintedFloat(self.w),
        )
    }
}

/*
pub fn vec4(x:f32, y:f32, z:f32, w:f32)->Vec4{
    Vec4{x:x, y:y, z:z, w:w}
}*/


impl Mat4 {
    pub fn identity() -> Mat4 {
        return Mat4 {v: [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0
        ]}
    }
    
    pub fn from_transform(transform: Transform) -> Mat4 {
        let q = transform.orientation;
        let t = transform.position;
        return Mat4 {v: [
            (1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z),
            (2.0 * q.x * q.y - 2.0 * q.z * q.w),
            (2.0 * q.x * q.z + 2.0 * q.y * q.w),
            0.0,
            (2.0 * q.x * q.y + 2.0 * q.z * q.w),
            (1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z),
            (2.0 * q.y * q.z - 2.0 * q.x * q.w),
            0.0,
            (2.0 * q.x * q.z - 2.0 * q.y * q.w),
            (2.0 * q.y * q.z + 2.0 * q.x * q.w),
            (1.0 - 2.0 * q.x * q.x - 2.0 * q.y * q.y),
            0.0,
            t.x,
            t.y,
            t.z,
            1.0
        ]}
    }
    
    pub fn rotate_tsrt(t1: Vec3, s: f32, r: Vec3, t2: Vec3) -> Mat4 {
        const TORAD: f32 = 0.017453292519943295;
        let cx = f32::cos(r.x * TORAD);
        let cy = f32::cos(r.y * TORAD);
        let cz = f32::cos(r.z * TORAD);
        let sx = f32::sin(r.x * TORAD);
        let sy = f32::sin(r.y * TORAD);
        let sz = f32::sin(r.z * TORAD);
        let m0 = s * (cy * cz + sx * sy * sz);
        let m1 = s * (-sz * cy + cz * sx * sy);
        let m2 = s * (sy * cx);
        let m4 = s * (sz * cx);
        let m5 = s * (cx * cz);
        let m6 = s * (-sx);
        let m8 = s * (-sy * cz + cy * sx * sz);
        let m9 = s * (sy * sz + cy * sx * cz);
        let m10 = s * (cx * cy);
        return Mat4 {v: [
            m0,
            m4,
            m8,
            0.0,
            m1,
            m5,
            m9,
            0.0,
            m2,
            m6,
            m10,
            0.0,
            t2.x + (m0 * t1.x + m1 * t1.y + m1 * t1.z),
            t2.y + (m4 * t1.x + m5 * t1.y + m6 * t1.z),
            t2.z + (m8 * t1.x + m9 * t1.y + m10 * t1.z),
            1.0
        ]}
    }
    
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let f = 1.0 / f32::tan(fov_y / 2.0);
        let nf = 1.0 / (near - far);
        return Mat4 {v: [
            f / aspect,
            0.0,
            0.0,
            0.0,
            0.0,
            f,
            0.0,
            0.0,
            0.0,
            0.0,
            (far + near) * nf,
            -1.0,
            0.0,
            0.0,
            (2.0 * far * near) * nf,
            0.0
        ]}
    }
    
    pub fn scale_translate(s: f32, x: f32, y: f32, z: f32) -> Mat4 {
        return Mat4 {v: [
            s,
            0.0,
            0.0,
            0.0,
            0.0,
            s,
            0.0,
            0.0,
            0.0,
            0.0,
            s,
            0.0,
            x,
            y,
            z,
            1.0
        ]}
        
    }
    
    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32, scalex: f32, scaley: f32) -> Mat4 {
        let lr = 1.0 / (left - right);
        let bt = 1.0 / (bottom - top);
        let nf = 1.0 / (near - far);
        /*return Mat4{v:[
            -2.0 * lr * scalex, 0.0, 0.0, (left+right) * lr,
            0.0, -2.0 * bt * scaley, 0.0, (top+bottom) * bt,
            0.0, 0.0, 2.0 * nf, (far + near) * nf,
            0.0, 0.0, 0.0, 1.0
        ]}*/
        return Mat4 {v: [
            -2.0 * lr * scalex,
            0.0,
            0.0,
            0.0,
            0.0,
            -2.0 * bt * scaley,
            0.0,
            0.0,
            0.0,
            0.0,
            -1.0 * nf,
            0.0,
            (left + right) * lr,
            (top + bottom) * bt,
            0.5 + (far + near) * nf,
            1.0
        ]}
    }
    
    pub fn transform_vec4(&self, v: Vec4) -> Vec4 {
        let m = &self.v;
        Vec4 {
            x: m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12] * v.w,
            y: m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13] * v.w,
            z: m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14] * v.w,
            w: m[3] * v.x + m[7] * v.y + m[11] * v.z + m[15] * v.w
        }
    }
    
    pub fn from_mul(a: &Mat4, b: &Mat4) -> Mat4 {
        // this is probably stupid. Programmed JS for too long.
        let a = &a.v;
        let b = &b.v;
        fn d(i: &[f32; 16], x: usize, y: usize) -> f32 {return i[x + 4 * y]}
        Mat4 {
            v: [
                d(a, 0, 0) * d(b, 0, 0) + d(a, 1, 0) * d(b, 0, 1) + d(a, 2, 0) * d(b, 0, 2) + d(a, 3, 0) * d(b, 0, 3),
                d(a, 0, 0) * d(b, 1, 0) + d(a, 1, 0) * d(b, 1, 1) + d(a, 2, 0) * d(b, 1, 2) + d(a, 3, 0) * d(b, 1, 3),
                d(a, 0, 0) * d(b, 2, 0) + d(a, 1, 0) * d(b, 2, 1) + d(a, 2, 0) * d(b, 2, 2) + d(a, 3, 0) * d(b, 2, 3),
                d(a, 0, 0) * d(b, 3, 0) + d(a, 1, 0) * d(b, 3, 1) + d(a, 2, 0) * d(b, 3, 2) + d(a, 3, 0) * d(b, 3, 3),
                d(a, 0, 1) * d(b, 0, 0) + d(a, 1, 1) * d(b, 0, 1) + d(a, 2, 1) * d(b, 0, 2) + d(a, 3, 1) * d(b, 0, 3),
                d(a, 0, 1) * d(b, 1, 0) + d(a, 1, 1) * d(b, 1, 1) + d(a, 2, 1) * d(b, 1, 2) + d(a, 3, 1) * d(b, 1, 3),
                d(a, 0, 1) * d(b, 2, 0) + d(a, 1, 1) * d(b, 2, 1) + d(a, 2, 1) * d(b, 2, 2) + d(a, 3, 1) * d(b, 2, 3),
                d(a, 0, 1) * d(b, 3, 0) + d(a, 1, 1) * d(b, 3, 1) + d(a, 2, 1) * d(b, 3, 2) + d(a, 3, 1) * d(b, 3, 3),
                d(a, 0, 2) * d(b, 0, 0) + d(a, 1, 2) * d(b, 0, 1) + d(a, 2, 2) * d(b, 0, 2) + d(a, 3, 2) * d(b, 0, 3),
                d(a, 0, 2) * d(b, 1, 0) + d(a, 1, 2) * d(b, 1, 1) + d(a, 2, 2) * d(b, 1, 2) + d(a, 3, 2) * d(b, 1, 3),
                d(a, 0, 2) * d(b, 2, 0) + d(a, 1, 2) * d(b, 2, 1) + d(a, 2, 2) * d(b, 2, 2) + d(a, 3, 2) * d(b, 2, 3),
                d(a, 0, 2) * d(b, 3, 0) + d(a, 1, 2) * d(b, 3, 1) + d(a, 2, 2) * d(b, 3, 2) + d(a, 3, 2) * d(b, 3, 3),
                d(a, 0, 3) * d(b, 0, 0) + d(a, 1, 3) * d(b, 0, 1) + d(a, 2, 3) * d(b, 0, 2) + d(a, 3, 3) * d(b, 0, 3),
                d(a, 0, 3) * d(b, 1, 0) + d(a, 1, 3) * d(b, 1, 1) + d(a, 2, 3) * d(b, 1, 2) + d(a, 3, 3) * d(b, 1, 3),
                d(a, 0, 3) * d(b, 2, 0) + d(a, 1, 3) * d(b, 2, 1) + d(a, 2, 3) * d(b, 2, 2) + d(a, 3, 3) * d(b, 2, 3),
                d(a, 0, 3) * d(b, 3, 0) + d(a, 1, 3) * d(b, 3, 1) + d(a, 2, 3) * d(b, 3, 2) + d(a, 3, 3) * d(b, 3, 3),
            ]
        }
    }
}
