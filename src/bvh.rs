use crate::aabb::{AABB, AABB_EMPTY};
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Lambertian;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utility::random_int_range;
use crate::vec3::Vec3;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}
impl BvhNode {
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: u32) -> Ordering {
        a.bounding_box()
            .axis_interval(axis_index)
            .min
            .partial_cmp(&b.bounding_box().axis_interval(axis_index).min)
            .unwrap()
    }
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
    pub fn new(src_objects: &mut Vec<Arc<dyn Hittable>>, s: usize, e: usize) -> Self {
        let mut bbox = AABB_EMPTY.clone();
        for object_index in (s..e) {
            bbox = AABB::new_aabb(&bbox, &(src_objects[object_index].bounding_box()));
        }
        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let objects = src_objects;
        let object_span = e - s;
        if object_span == 1 {
            BvhNode {
                left: objects[s].clone(),
                right: objects[s].clone(),
                bbox,
            }
        } else if object_span == 2 {
            if comparator(&objects[s], &objects[s + 1]) == Ordering::Less {
                BvhNode {
                    left: objects[s].clone(),
                    right: objects[s + 1].clone(),
                    bbox,
                }
            } else {
                BvhNode {
                    left: objects[s + 1].clone(),
                    right: objects[s].clone(),
                    bbox,
                }
            }
        } else {
            (*objects)[s..e].sort_by(comparator);
            let mid = s + object_span / 2;
            let left = Arc::new(Self::new(objects, s, mid));
            let right = Arc::new(Self::new(objects, mid, e));
            Self { left, right, bbox }
        }
    }
    pub fn new_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }
}
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let ray_t = ray_t.clone();
        if !self.bbox.hit(r, &ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, &ray_t, rec);
        let mut hit_right = false;
        if hit_left {
            hit_right = self.right.hit(r, &Interval::new(ray_t.min, rec.t), rec);
        } else {
            hit_right = self.right.hit(r, &ray_t, rec);
        }
        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
