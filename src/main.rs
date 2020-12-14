use std::error::Error;

use pbrt::{Camera, Material, Object, Renderer, Scene, Sphere};

fn main() -> Result<(), Box<dyn Error>> {
    let mut scene = Scene::default();

    scene.add_object(Object::new(Sphere::default(), Material::default()).translate(0.0, 0.0, -3.0));

    let r = Renderer::new(scene, Camera::default(), 500, 500, 50);
    r.render().save("output.png")?;

    Ok(())
}
