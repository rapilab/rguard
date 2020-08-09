use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use rguard_core::classfile::library_clazz::LibraryClazz;
use rguard_core::classfile::program_class::ProgramClass;
use rguard_core::classfile::visitor::class_visitor::ClassVisitor;
use rguard_core::classfile::clazz::Clazz;

pub struct UsedClassFilter {
    pub simple_usage_marker: SimpleUsageMarker,
    pub used_class_visitor: Box<dyn ClassVisitor>,
}

impl UsedClassFilter {
    pub fn new(
        simple_usage_marker: SimpleUsageMarker,
        used_class_visitor: Box<dyn ClassVisitor>,
    ) -> UsedClassFilter {
        UsedClassFilter {
            simple_usage_marker,
            used_class_visitor,
        }
    }
}

impl ClassVisitor for UsedClassFilter {
    fn visit_any_class(&self, clazz: Box<dyn Clazz>) {
        unimplemented!()
    }

    fn visit_program_class(&self, program_clazz: ProgramClass) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
