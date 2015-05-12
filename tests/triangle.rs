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

#[macro_use]
extern crate cgmath;

use cgmath::*;

#[test]
fn test_ray_intersection() {
    let tri: Triangle<Point3<f32>> = Triangle::new(
        Point3::new( 0.4, -1.8, -1.5),
        Point3::new(-0.1,  1.0,  1.2),
        Point3::new(-2.0, -1.0, -0.2)
    );
    
    let ray: Ray3<f32> = Ray3::from_points(
        Point3::new( 0.4, -2.3,  2.3),
        Point3::new(-1.2, -0.2, -1.8)
    ).normalize();
    
    let inter = (&ray, &tri).intersection().unwrap();
    assert_approx_eq_eps!(inter.x, -0.663337, 0.000001);
    assert_approx_eq_eps!(inter.y, -0.904370, 0.000001);
    assert_approx_eq_eps!(inter.z, -0.424801, 0.000001);
    
    let ray: Ray3<f32> = Ray3::new(
        Point3::new(  0.4, -2.3,  2.3),
        Vector3::new( 0.6, -0.2, -1.8)
    ).normalize();
    
    let inter = (&ray, &tri).intersection();
    assert_eq!(None, inter);
}