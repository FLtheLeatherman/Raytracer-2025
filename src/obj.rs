use crate::bvh;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material};
use crate::texture::{ImageTexture, SolidColor, Texture, UV};
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use std::path::Path;
use std::sync::Arc;

pub fn load_model(obj_filename: &str, scale: f64) -> HittableList {
    let mut world = HittableList::new();
    let obj_path = format!("assets/{}", obj_filename);
    let (models, materials) = tobj::load_obj(
        &obj_path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        },
    )
    .expect("Failed to load OBJ file");
    let model_dir = Path::new(&obj_path)
        .parent()
        .expect("Failed to get model directory");
    let mut loaded_materials: Vec<Arc<dyn Material>> = Vec::new();
    if let Ok(materials) = materials {
        println!("Found {} materials in .mtl file.", materials.len());
        for m in materials {
            let albedo: Arc<dyn Texture> = if let Some(texture_name) = &m.diffuse_texture {
                let texture_path = model_dir.join(texture_name);
                println!("Loading texture: {:?}", texture_path);
                Arc::new(ImageTexture::new(texture_path.to_str().unwrap()))
            } else {
                let diffuse = m.diffuse.unwrap_or([0.73, 0.73, 0.73]);
                let color = Color::new(diffuse[0] as f64, diffuse[1] as f64, diffuse[2] as f64);
                Arc::new(SolidColor::new_color(&color))
            };
            loaded_materials.push(Arc::new(Lambertian::new_tex(albedo)));
        }
    }
    let default_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.0, 0.8)));
    println!("Loading {} models...", models.len());
    for model in models {
        let mesh = &model.mesh;
        println!("Loading {} models...", mesh.indices.len());
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            let p0 = Vec3::new(
                mesh.positions[3 * i0] as f64,
                mesh.positions[3 * i0 + 1] as f64,
                mesh.positions[3 * i0 + 2] as f64,
            ) * scale;
            let p1 = Vec3::new(
                mesh.positions[3 * i1] as f64,
                mesh.positions[3 * i1 + 1] as f64,
                mesh.positions[3 * i1 + 2] as f64,
            ) * scale;
            let p2 = Vec3::new(
                mesh.positions[3 * i2] as f64,
                mesh.positions[3 * i2 + 1] as f64,
                mesh.positions[3 * i2 + 2] as f64,
            ) * scale;
            let uvs = if !mesh.texcoords.is_empty() {
                UV::new(
                    Vec3::new(
                        mesh.texcoords[2 * i0] as f64,
                        mesh.texcoords[2 * i1] as f64,
                        mesh.texcoords[2 * i2] as f64,
                    ),
                    Vec3::new(
                        mesh.texcoords[2 * i0 + 1] as f64,
                        mesh.texcoords[2 * i1 + 1] as f64,
                        mesh.texcoords[2 * i2 + 1] as f64,
                    ),
                )
            } else {
                UV::default()
            };
            let mat = match mesh.material_id {
                Some(mat_id) => loaded_materials[mat_id].clone(),
                None => default_material.clone(),
            };
            world.add(Arc::new(Triangle::new(
                &p0,
                &(p1 - p0),
                &(p2 - p0),
                mat,
                uvs,
            )));
        }
    }
    println!("done model loading!");
    let bvh_node = bvh::BvhNode::new_list(&mut world);
    let mut final_world = HittableList::new();
    final_world.add(Arc::new(bvh_node));
    final_world
}
