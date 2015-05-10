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

use std::marker::PhantomData;
use num::{BaseNum, BaseFloat};
use rust_num::zero;
use point::{Point, Point2, Point3};
use vector::{Vector, EuclideanVector, Vector2, Vector3};

/// A generic ray starting at `origin` and extending infinitely in
/// `direction`.
#[derive(Copy, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Ray<S, P, V> {
    pub origin: P,
    pub direction: V,
    phantom_s: PhantomData<S>
}

impl<S: BaseNum, V: Vector<S>, P: Point<S, V>> Ray<S, P, V> {
    pub fn new(origin: P, direction: V) -> Ray<S, P, V> {
        Ray {
        	origin: origin,
        	direction: direction,
        	phantom_s: PhantomData
        }
    }
}

pub type Ray2<S> = Ray<S, Point2<S>, Vector2<S>>;
pub type Ray3<S> = Ray<S, Point3<S>, Vector3<S>>;

impl<S: BaseNum> Ray3<S> {
    pub fn at(&self, t: S) -> Point3<S> {
        Point3 {x: self.x(t), y: self.y(t), z: self.z(t)}
    }
    
    pub fn x(&self, t: S) -> S {
        self.direction.x * t + self.origin.x
    }
    
    pub fn y(&self, t: S) -> S {
        self.direction.y * t + self.origin.y
    }
    
    pub fn z(&self, t: S) -> S {
        self.direction.z * t + self.origin.z
    }
    
    // Returns a t value.
    pub fn where_x_eq(&self, x: S) -> Option<S> {
        if self.direction.x == zero() {
            None
        } else {
            Some((x - self.origin.x) / self.direction.x)
        }
    }
    
    // Returns a t value.
    pub fn where_y_eq(&self, y: S) -> Option<S> {
        if self.direction.y == zero() {
            None
        } else {
            Some((y - self.origin.y) / self.direction.y)
        }
    }
    
    // Returns a t value.
    pub fn where_z_eq(&self, z: S) -> Option<S> {
        if self.direction.z == zero() {
            None
        } else {
            Some((z - self.origin.z) / self.direction.z)
        }
    }
}

impl<S: BaseFloat, V: EuclideanVector<S>, P: Point<S, V>> Ray<S, P, V> {
    pub fn normalize(&self) -> Ray<S, P, V> {
        Ray::new(self.origin.clone(), self.direction.normalize())
    } 
}