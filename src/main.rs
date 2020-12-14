use std::error::Error;

use pbrt::{Camera, Material, Object, Renderer, Scene, Sphere};

fn main() -> Result<(), Box<dyn Error>> {
    let mut scene = Scene::default();

    scene.add_object(Object::new(Sphere::default(), Material::default()).translate(0.0, 0.0, -3.0));
    scene.add_object(Object::new(Sphere::default(), Material::default()).translate(3.0, 3.0, -5.0));

    let r = Renderer::new(scene, Camera::default(), 1000, 1000, 20);
    r.render().save("output.png")?;

    Ok(())
}
