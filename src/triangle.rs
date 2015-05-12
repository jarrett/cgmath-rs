// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num::{BaseFloat};
use rust_num::{zero, one};

use vector::{Vector, Vector3};
use point::{Point, Point3};
use ray::Ray3;
use intersect::Intersect;
use approx::ApproxEq;

pub struct Triangle<P> {
    pub p0: P,
    pub p1: P,
    pub p2: P
}

impl<P> Triangle<P> {
    pub fn new(p0: P, p1: P, p2: P) -> Triangle<P> {
        Triangle {p0: p0, p1: p1, p2: p2}
    }
}

impl<'a, 'b, S: BaseFloat> Intersect<Option<Point3<S>>> for (&'a Ray3<S>, &'b Triangle<Point3<S>>) {
    fn intersection(&self) -> Option<Point3<S>> {
        let (ray, tri) = *self;
        
        let edge1: Vector3<S> = tri.p1.sub_p(&tri.p0);
        let edge2: Vector3<S> = tri.p2.sub_p(&tri.p0);
    
        let pvec: Vector3<S> = ray.direction.cross(&edge2);
        
        let det: S = edge1.dot(&pvec);
        
        if det.approx_eq(&zero()) { return None; }
        let inv_det: S = one::<S>() / det;
    
        let tvec: Vector3<S> = ray.origin.sub_p(&tri.p0);
        
        let u: S = tvec.dot(&pvec) * inv_det;
        if u < zero() || u > one() { return None; }
    
        let qvec: Vector3<S> = tvec.cross(&edge1);
    
        let v: S = ray.direction.dot(&qvec) * inv_det;
        if v < zero() || u + v > one() { return None; }
    
        let t: S = edge2.dot(&qvec) * inv_det;
        Some(ray.at(t))
    }
}

impl<'a, 'b, S: BaseFloat> Intersect<Option<Point3<S>>> for (Ray3<S>, Triangle<Point3<S>>) {
    fn intersection(&self) -> Option<Point3<S>> {
        let (ref ray, ref tri) = *self;
        (ray, tri).intersection()
    }
}