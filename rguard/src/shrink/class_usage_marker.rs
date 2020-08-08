use rguard_core::classfile::library_clazz::LibraryClazz;
use rguard_core::classfile::program_class::ProgramClass;
use rguard_core::classfile::visitor::class_visitor::ClassVisitor;

pub struct ClassUsageMarker {}

impl Default for ClassUsageMarker {
    fn default() -> Self {
        ClassUsageMarker {}
    }
}

impl ClassVisitor for ClassUsageMarker {
    fn visit_program_class(&self, program_clazz: ProgramClass) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
