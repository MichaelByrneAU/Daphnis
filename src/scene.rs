use crate::camera::Camera;
use crate::objects::ObjectList;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub camera: Camera,
    pub world: ObjectList,
}

impl Scene {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, world: ObjectList) -> Scene {
        Scene {
            width,
            height,
            samples,
            camera,
            world,
        }
    }
}
