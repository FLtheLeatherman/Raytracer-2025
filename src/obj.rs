use crate::bvh;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material};
use crate::texture::{ImageTexture, SolidColor, Texture, UV}; // 引入新 Texture
use crate::triangle::Triangle;
use crate::vec3::Vec3; // 假设有 Vec2
use std::path::Path;
use std::sync::Arc;
use tobj;

pub fn load_model(obj_filename: &str, scale: f64) -> HittableList {
    let mut world = HittableList::new();
    // 强烈建议开启 triangulate，这会极大简化你的代码
    // tobj 会自动将多边形面转换为三角形
    let obj_path = format!("assets/{}", obj_filename);
    let (models, materials) = tobj::load_obj(
        &obj_path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true, // 设置为 true
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load OBJ file");
    // 获取 OBJ 文件所在的目录，用于解析相对路径的纹理
    let model_dir = Path::new(&obj_path)
        .parent()
        .expect("Failed to get model directory");
    let mut loaded_materials: Vec<Arc<dyn Material>> = Vec::new();
    if let Ok(materials) = materials {
        println!("Found {} materials in .mtl file.", materials.len());
        for m in materials {
            // 对于 .mtl 文件中的每一种材质，我们只加载一次
            let albedo: Arc<dyn Texture> = if let Some(texture_name) = &m.diffuse_texture {
                // 找到了纹理贴图！
                let texture_path = model_dir.join(texture_name);
                println!("Loading texture: {:?}", texture_path);
                Arc::new(ImageTexture::new(texture_path.to_str().unwrap()))
            } else {
                // 没找到纹理，就使用漫反射颜色
                let diffuse = m.diffuse.unwrap_or([0.73, 0.73, 0.73]); // 提供一个默认颜色
                let color = Color::new(diffuse[0] as f64, diffuse[1] as f64, diffuse[2] as f64);
                Arc::new(SolidColor::new_color(&color))
            };
            // 创建 Lambertian 材质并存入我们的缓存 Vec 中
            loaded_materials.push(Arc::new(Lambertian::new_tex(albedo)));
        }
    }
    // 创建一个默认材质，用于处理没有指定材质ID的面
    let default_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.0, 0.8))); // 亮粉色，方便调试
    println!("Loading {} models...", models.len());
    for model in models {
        let mesh = &model.mesh;
        // tobj 以 (v, vt, vn) 的形式交错排列索引。
        // 因为我们设置了 triangulate: true，所以 indices 的长度总是3的倍数。
        println!("Loading {} models...", mesh.indices.len());
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            // 提取顶点位置
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
            // 提取 UV 坐标
            // 检查 mesh.texcoords 是否为空
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
                // 如果模型没有 UV 坐标，提供默认值
                UV::default()
            };
            let mat = match mesh.material_id {
                Some(mat_id) => loaded_materials[mat_id].clone(), // clone() 只复制 Arc 指针，非常快！
                None => default_material.clone(),                 // 如果面没有材质，使用默认材质
            };
            // 创建并添加三角形
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
    // 你的 BVH 构建逻辑保持不变
    let bvh_node = bvh::BvhNode::new_list(&mut world);
    let mut final_world = HittableList::new();
    final_world.add(Arc::new(bvh_node));
    final_world
}
