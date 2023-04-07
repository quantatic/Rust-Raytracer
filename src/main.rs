mod material;
mod ray;
mod shape;

use anyhow::Result;
use eframe::egui;

use egui::{
    color_picker::color_edit_button_rgb, ColorImage, Context, ImageData, Rgba, Slider,
    TextureHandle, TextureOptions,
};
use material::{DiffuseMaterial, LightMaterial, Material, MetalMaterial};
use nalgebra::{UnitVector3, Vector3};
use rand::Rng;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use shape::ShapeCollisionInformation;
use shape::{Plane, Shape, Sphere};

const RENDER_WIDTH: usize = 250;
const RENDER_HEIGHT: usize = 250;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|creation_ctx| {
            let mut result = MyEguiApp::new(&creation_ctx.egui_ctx).unwrap();
            result.do_render(&creation_ctx.egui_ctx);
            Box::new(result)
        }),
    )
    .unwrap();
}

struct MyEguiApp {
    shapes: Vec<Box<dyn Shape + Sync>>,
    num_samples: usize,
    image: TextureHandle,
}

impl MyEguiApp {
    fn new(ctx: &Context) -> Result<Self> {
        let shapes: Vec<Box<dyn Shape + Sync>> = vec![
            Box::new(Plane {
                point: Vector3::new(0.0, -3.0, 0.0),
                normal: UnitVector3::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
                material: DiffuseMaterial {
                    albedo: Vector3::new(1.0, 1.0, 1.0),
                },
            }),
            Box::new(Sphere {
                center: Vector3::new(1.0, 2.0, -4.0),
                radius: 1.0,
                material: DiffuseMaterial {
                    albedo: Vector3::new(0.1, 1.0, 0.1),
                },
            }),
            Box::new(Sphere {
                center: Vector3::new(-1.0, 2.0, -4.0),
                radius: 1.0,
                material: LightMaterial {
                    color: Vector3::new(0.9, 0.9, 0.9),
                },
            }),
        ];

        let mut result = Self {
            shapes,
            num_samples: 10,
            image: ctx.load_texture("init-texture", ColorImage::example(), Default::default()),
        };
        result.do_render(ctx);
        Ok(result)
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Render Window").show(ctx, |ui| {
            ui.image(&self.image, ui.available_size());
        });

        let mut needs_render = false;
        egui::Window::new("Shapes Settings").show(ctx, |ui| {
            for (i, shape) in self.shapes.iter_mut().enumerate() {
                ui.collapsing(format!("shape {i}"), |ui| {
                    needs_render |= shape.render(ui);

                    ui.horizontal(|ui| {
                        ui.label("Color");

                        let old_color = shape.material().color();

                        let mut rgb = [old_color.x, old_color.y, old_color.z];
                        if color_edit_button_rgb(ui, &mut rgb).changed() {
                            needs_render |= true;

                            let new_color = Vector3::new(rgb[0], rgb[1], rgb[2]);
                            *shape.material_mut().color_mut() = new_color;
                        }
                    });
                });
            }
        });

        egui::Window::new("Render Settings").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Number of Samples");
                needs_render |= ui
                    .add(Slider::new(&mut self.num_samples, 1..=1_000).logarithmic(true))
                    .changed();
            });
        });

        if needs_render {
            self.do_render(ctx);
        }
    }
}

impl MyEguiApp {
    const MAX_REFLECTION_DEPTH: usize = 5;

    const BLACK: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    const WHITE: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
    const BLUE: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

    fn do_render(&mut self, ctx: &Context) {
        let pixels = (0..(RENDER_WIDTH * RENDER_HEIGHT))
            .into_par_iter()
            .map(|idx| {
                let x = idx % RENDER_WIDTH;
                let y = idx / RENDER_WIDTH;

                let mut rng = rand::thread_rng();

                let result = (0..self.num_samples)
                    .map(|_| {
                        let offset_x = rng.gen_range(-0.5..0.5);
                        let offset_y = rng.gen_range(-0.5..0.5);

                        let actual_x = (x as f32) + offset_x;
                        let actual_y = (y as f32) + offset_y;

                        let ray_x = ((actual_x / (RENDER_WIDTH as f32)) * 2.0) - 1.0;
                        let ray_y = -(((actual_y / (RENDER_HEIGHT as f32)) * 2.0) - 1.0);

                        // cast from origin to -1 in z, [-1, 1] in x, y
                        let ray = Ray {
                            pos: Vector3::new(0.0, 0.0, 0.0),
                            dir: UnitVector3::new_normalize(Vector3::new(ray_x, ray_y, -1.0)),
                        };

                        self.cast_ray(ray, Self::MAX_REFLECTION_DEPTH)
                    })
                    .sum::<Vector3<f32>>()
                    / (self.num_samples as f32);
                Rgba::from_rgb(result.x, result.y, result.z).into()
            })
            .collect::<Vec<_>>();

        let image = ImageData::Color(ColorImage {
            pixels,
            size: [RENDER_WIDTH, RENDER_HEIGHT],
        });

        self.image = ctx.load_texture("my-test-texture", image, TextureOptions::NEAREST);
    }

    fn cast_ray(&self, ray: Ray, max_depth: usize) -> Vector3<f32> {
        if max_depth == 0 {
            return Self::BLACK;
        }

        if let Some((shape_collision, closest_shape)) = self
            .shapes
            .iter()
            .filter_map(|shape| shape.intersect(ray).map(|info| (info, shape)))
            .min_by(|(info_1, _), (info_2, _)| f32::total_cmp(&info_1.distance, &info_2.distance))
        {
            let material_collision = closest_shape.material().evaluate(ray, shape_collision);

            let reflection_light =
                if let Some(reflection_info) = material_collision.reflection_information {
                    let reflection_ray = Ray {
                        pos: shape_collision.intersection_point,
                        dir: reflection_info.reflection,
                    };

                    let recursive_color = self.cast_ray(reflection_ray, max_depth - 1);
                    reflection_info.albedo.component_mul(&recursive_color)
                } else {
                    Vector3::zeros()
                };

            let emission_light =
                if let Some(emission_info) = material_collision.emission_information {
                    emission_info.color
                } else {
                    Vector3::zeros()
                };

            reflection_light + emission_light
        } else {
            // 0.0 to 1.0
            // let y = (ray.dir.into_inner().y + 1.0) / 2.0;
            // (Self::WHITE * y) + (Self::BLUE * (1.0 - y))
            Self::BLACK
        }
    }
}
