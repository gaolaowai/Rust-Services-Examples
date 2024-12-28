//! 
//! Intent is to have a single resource that can be given different labels
//! 

use nanoserde::{DeBin, SerBin};

#[derive(DeBin, SerBin, Clone)]
struct Resource {
    name: String,
    description: String,
    roles_for_reading: u128,
    roles_for_writing: u128,
    attribute_fields: Vec<Attributes>
}

#[derive(DeBin, SerBin, Clone)]
enum Attributes {
    Stringfield(String),
    Numberfield(i32),
    Floatfield(f32)
}

#[derive(DeBin, SerBin, Clone)]
struct Resources {
    resources: Vec<Resource>,
    resource_status: Vec<u8> // 1 --> enabled, 0 --> disabled
}
const RESOURCE_DISABLED: u8 = 0u8;
const RESOURCE_ENABLED: u8 = 1u8;

impl Resources {
    pub fn list_resources(&self) -> Vec<Resource> {
        self.resources.clone()
    }
}