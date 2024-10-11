use gust_core::objects::intermediaries::wavefront_object::WavefrontObject;
use gust_math::matrices::mat3::Mat3;
use gust_math::vectors::vect3::Vect3;

#[derive(Copy, Clone, Debug)]
pub struct ColliderComponentImpl {
    pub collider_type: ColliderType,
}

#[derive(Copy, Clone, Debug)]
pub enum ColliderType {
    Sphere { radius: f32 },
    AABB { min: Vect3, max: Vect3 },
    OBB { center: Vect3, axes: Mat3, half_extents: Vect3 },
}

impl ColliderComponentImpl {
    pub fn from_wavefront_aabb(wavefront: &WavefrontObject) -> Self {
        let mut min = Vect3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vect3::new(f32::MIN, f32::MIN, f32::MIN);

        for vertex in &wavefront.vertices {
            let v = Vect3::new(vertex[0], vertex[1], vertex[2]);

            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);

            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        ColliderComponentImpl {
            collider_type: ColliderType::AABB { min, max },
        }
    }
}
