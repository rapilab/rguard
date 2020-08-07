use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;
use crate::infra::print_writer::PrintWriter;
use rguard_core::classfile::visitor::class_cleaner::ClassCleaner;
use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use crate::shrink::class_usage_marker::ClassUsageMarker;
use crate::shrink::usage_marker::UsageMarker;

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

        let simple_usage_marker = SimpleUsageMarker::default();
        let class_usage_marker = ClassUsageMarker::default();

        let usage_marker = UsageMarker::new(self.configuration);
        usage_marker.mark(
            program_class_pool,
            library_class_pool,
            resource_file_pool,
            simple_usage_marker,
            class_usage_marker
        )
    }
}
