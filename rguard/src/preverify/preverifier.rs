use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::classfile::visitor::class_cleaner::ClassCleaner;
use crate::preverify::code_preverifier::CodePreverifier;

pub struct Preverifier {
    configuration: Configuration
}

impl Preverifier {
    pub fn new(configuration: Configuration) -> Preverifier {
        Preverifier { configuration }
    }
    pub fn execute(&self, program_class_pool: ClassPool) {
        program_class_pool.classes_accept(Box::from(ClassCleaner::default()));
        CodePreverifier::new();
    }
}
