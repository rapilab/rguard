use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;

pub struct Shrinker {
    configuration: Configuration
}

impl Shrinker {
    pub fn new(configuration: Configuration) -> Self {
        Shrinker { configuration }
    }
    pub fn execute(&self, program_class_pool: ClassPool, library_class_pool: ClassPool, resource_file_pool: ResourceFilePool) {}
}
