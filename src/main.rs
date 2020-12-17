use pbrt::*;

use nalgebra::{Point3, Vector3};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for i in 0..100 {
        let theta = std::f64::consts::TAU * ((i as f64) / (100.0));

        let mut scene = Scene::default();

        scene.add_object(
            Object::new(Sphere::default(), Material::default()).translate(-1.5, 0.0, -10.0),
        );
        scene.add_object(
            Object::new(Sphere::default(), Material::default()).translate(1.5, 0.0, -10.0),
        );
        scene.add_object(
            Object::new(Sphere::default(), Material::default()).translate(0.0, 2.0, -10.0),
        );

        scene.add_object(
            Object::new(Plane::new(Vector3::y_axis()), Material::default())
                .translate(0.0, -1.3, 0.0),
        );

        scene.add_light(PointLight::new(
            Point3::new(0.0, 13.0, -10.0),
            Color::hex(0xFF0000),
        ));

        let r = Renderer::new(
            scene,
            Camera::new(
                Point3::new(10.0 * theta.sin(), 5.0, (10.0 * theta.cos()) - 10.0),
                Point3::new(0.0, 0.0, -10.0),
                Vector3::y_axis(),
                std::f64::consts::FRAC_PI_2,
                1.0,
            ),
            1000,
            1000,
            25,
        );

        let filename = format!("outputs/output_{:03}.png", i);

        println!("Rendering: {}", filename);
        r.render().save(&filename)?;
        println!("Saved: {}", filename);
    }

    Ok(())
}
