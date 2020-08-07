use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;

pub struct Obfuscator {
    configuration: Configuration
}

impl Obfuscator {
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }
    pub fn execute(&self, program_class_pool: ClassPool, library_class_pool: ClassPool, resource_file_pool: ResourceFilePool) {

    }
}

