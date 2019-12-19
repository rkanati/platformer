
pub type V2 = nalgebra::Vector2<f32>;
pub type Vu2 = nalgebra::Unit<V2>;
pub type P2 = nalgebra::Point2<f32>;

pub const EPSILON: f32 = 0.00001;

pub trait V2Ext {
    fn left(&self) -> V2;
    fn unit(&self) -> Vu2;
    fn unit_and_norm(&self) -> (Vu2, f32);
}

impl V2Ext for V2 {
    fn left(&self) -> V2 {
        V2::new(-self.y, self.x)
    }

    fn unit(&self) -> Vu2 {
        Vu2::new_normalize(*self)
    }

    fn unit_and_norm(&self) -> (Vu2, f32) {
        Vu2::new_and_get(*self)
    }
}

pub trait Linear2 {
    fn whole_line(&self) -> Line2;
    fn parameter_on(&self, lambda: f32) -> bool;
    fn direction(&self) -> Vu2;
    fn project(&self, p: P2) -> f32 {
        let line = self.whole_line();
        let v = p - line.p;
        v.dot(&line.d)
    }
    fn intersect(&self, other: &impl Linear2) -> Option<P2> {
        let la = self.whole_line();
        let lb = other.whole_line();

        let offset = la.p - lb.p;
        let denom = lb.d.y * la.d.x - lb.d.x * la.d.y;
        if denom.abs () < EPSILON {
            None
        }
        else {
            let lambda = (lb.d.x * offset.y - lb.d.y * offset.x) / denom;
            let mu     = (la.d.y * offset.x - la.d.x * offset.y) / denom;

            if self.parameter_on(lambda) && other.parameter_on(mu) {
                let p = la.at(lambda);
                Some(p)
            }
            else {
                None
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line2 {
    pub p: P2,
    pub d: Vu2,
}

impl Line2 {
    fn new(p: P2, d: Vu2) -> Line2 {
        Line2 { p, d }
    }

    fn at(&self, lambda: f32) -> P2 {
        self.p + lambda * self.d.into_inner()
    }
}

impl Linear2 for Line2 {
    fn whole_line(&self) -> Line2 { *self }
    fn parameter_on(&self, _: f32) -> bool { true }
    fn direction(&self) -> Vu2 { self.d }
}



#[derive(Clone, Copy, Debug)]
pub struct Ray2(Line2);

impl Linear2 for Ray2 {
    fn whole_line(&self) -> Line2 { self.0 }
    fn parameter_on(&self, lambda: f32) -> bool { lambda >= 0.0 }
    fn direction(&self) -> Vu2 { self.0.direction() }
}

impl Ray2 {
    fn new(p: P2, dir: Vu2) -> Ray2 {
        Ray2(Line2::new(p, dir))
    }
}




#[derive(Clone, Copy, Debug)]
pub struct Seg2 {
    ray:  Ray2,
    dist: f32
}

impl Linear2 for Seg2 {
    fn whole_line(&self) -> Line2 { self.ray.whole_line() }
    fn parameter_on(&self, lambda: f32) -> bool { lambda >= 0.0 && lambda <= self.dist }
    fn direction(&self) -> Vu2 { self.ray.direction() }
}

impl Seg2 {
    fn new_from_ray(ray: Ray2, dist: f32) -> Seg2 {
        Seg2 { ray, dist }
    }

    fn new_from_points(a: P2, b: P2) -> Seg2 {
        let (dir, dist) = (b - a).unit_and_norm();
        let ray = Ray2::new(a, dir);
        Seg2 { ray, dist }
    }

    fn new_from_displacement(p: P2, d: V2) -> Seg2 {
        let (dir, dist) = d.unit_and_norm();
        let ray = Ray2::new(p, dir);
        Seg2 { ray, dist }
    }
}




#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    On,
    Left,
    Right
}

impl Into<usize> for Side {
    fn into(self) -> usize {
        match self {
            Side::On    => 0,
            Side::Left  => 1,
            Side::Right => 2
        }
    }
}

pub fn point_side_of_line(l: Line2, p: P2) -> Side {
    let off = p - l.p;
    let n = l.d.left().unit();
    let dp = n.dot(&off);
    if dp > EPSILON {
        Side::Left
    }
    else if dp < -EPSILON {
        Side::Right
    }
    else {
        Side::On
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub verts: Vec<V2>,
}

impl Shape {
    pub fn new(vs: &[V2]) -> Shape {
        Self::new_from_vec(vs.to_vec())
    }

    pub fn new_from_vec(verts: Vec<V2>) -> Shape {
        Shape { verts }
    }
}

pub fn shape_side_of_line(l: Line2, p: P2, s: &Shape) -> Side {
    let mut prev_side = Side::On;

    for v in &s.verts {
        let side = point_side_of_line(l, p + *v);
        if side == Side::On {
            continue;
        }
        else if prev_side == Side::On {
            prev_side = side;
        }
        else if prev_side != side {
            return Side::On
        }
    }

    prev_side
}

