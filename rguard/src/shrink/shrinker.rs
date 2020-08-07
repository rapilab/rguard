use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;
use crate::infra::print_writer::PrintWriter;
use rguard_core::classfile::visitor::class_cleaner::ClassCleaner;

pub struct Shrinker {
    configuration: Configuration
}

impl Shrinker {
    pub fn new(configuration: Configuration) -> Self {
        Shrinker { configuration }
    }
    pub fn execute(&self, program_class_pool: ClassPool, library_class_pool: ClassPool, resource_file_pool: ResourceFilePool) {
        let writer = PrintWriter::new();

        program_class_pool.classes_accept(ClassCleaner::default());
        library_class_pool.classes_accept(ClassCleaner::default());

    }
}
