mod geometry;
mod shapes;

use crate::geometry::{Ray, Vec3};
use crate::shapes::{Hitable, Sphere};

fn render((width, height): (u32, u32), scale: f64, objects: &[Sphere]) {
    let (width, height) = (width as i32, height as i32);

    for y in (-height / 2)..(height / 2) {
        let y = y as f64 / height as f64 * scale;
        for x in (-width / 2)..(width / 2) {
            let x = x as f64 / width as f64 * scale;

            let ray = Ray {
                pos: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                dir: Vec3 {
                    x,
                    y,
                    z: -1.0
                }
            };

            let mut hit = false;
            for obj in objects.iter() {
                if obj.hit(&ray) {
                    hit = true;
                    break;
                }
            }

            if hit {
                print!("##");
            } else {
                print!("  ");
            }
        }

        println!();
    }
}

fn main() {
    println!("Hello, world!");

    let objects = vec![
        Sphere {
            pos: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.5
            },
            radius: 1.0
        },
        Sphere {
            pos: Vec3 {
                x: 1.0,
                y: 1.0,
                z: -3.0
            },
            radius: 2.0
        }
    ];

    render((50, 50), 3.0, &objects);
}
