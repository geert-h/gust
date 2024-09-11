use std::collections::HashMap;

pub struct ImageStorage {
    images: HashMap<ImageId, String>,
}

pub struct ImageId(u32);