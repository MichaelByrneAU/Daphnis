use crate::camera::Camera;
use crate::objects::Object;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub camera: Camera,
    pub world: Object,
}

impl Scene {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, world: Object) -> Scene {
        Scene {
            width,
            height,
            samples,
            camera,
            world,
        }
    }
}
