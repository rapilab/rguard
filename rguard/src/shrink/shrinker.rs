use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;
use crate::infra::print_writer::PrintWriter;
use rguard_core::classfile::visitor::class_cleaner::ClassCleaner;
use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use crate::shrink::class_usage_marker::ClassUsageMarker;
use crate::shrink::usage_marker::UsageMarker;
use crate::shrink::used_class_filter::UsedClassFilter;
use rguard_core::classfile::visitor::multi_class_visitor::MultiClassVisitor;
use crate::shrink::class_shrinker::ClassShrinker;
use rguard_core::classfile::visitor::class_pool_filler::ClassPoolFiller;
use rguard_core::classfile::visitor::class_visitor::ClassVisitor;

pub struct Shrinker {
    configuration: Configuration
}

impl Shrinker {
    pub fn new(configuration: Configuration) -> Self {
        Shrinker { configuration }
    }
    pub fn execute(&self, program_class_pool: ClassPool, library_class_pool: ClassPool, resource_file_pool: ResourceFilePool) {
        let writer = PrintWriter::new();

        program_class_pool.classes_accept(Box::from(ClassCleaner::default()));
        library_class_pool.classes_accept(Box::from(ClassCleaner::default()));

        let simple_usage_marker = SimpleUsageMarker::default();
        let class_usage_marker = ClassUsageMarker::default();

        let usage_marker = UsageMarker::new(self.configuration);
        usage_marker.mark(
            program_class_pool,
            library_class_pool,
            resource_file_pool,
            simple_usage_marker,
            class_usage_marker
        );

        let new_program_class_pool = ClassPool::default();

        let mut visitors: Vec<Box<dyn ClassVisitor>> = vec![];
        let shrinker = ClassShrinker::new(simple_usage_marker);
        let filler = ClassPoolFiller::new(new_program_class_pool);
        visitors.push(Box::from(shrinker));
        visitors.push(Box::from(filler));

        let multi_visitors = MultiClassVisitor::new(visitors);
        let used_class_filter = UsedClassFilter::new(simple_usage_marker, Box::from(multi_visitors));
        program_class_pool.classes_accept(Box::from(used_class_filter));

    }
}
