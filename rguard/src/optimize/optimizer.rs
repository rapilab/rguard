use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;

pub struct Optimizer {
    configuration: Configuration,
}

impl Optimizer {
    pub fn new(configuration: Configuration) -> Optimizer {
        Optimizer { configuration }
    }

    pub fn execute(&self, program_class_pool: ClassPool, library_class_pool: ClassPool) {}
}
