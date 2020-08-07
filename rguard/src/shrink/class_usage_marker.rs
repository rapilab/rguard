use rguard_core::classfile::visitor::class_visitor::ClassVisitor;
use rguard_core::classfile::program_clazz::ProgramClazz;
use rguard_core::classfile::library_clazz::LibraryClazz;

pub struct ClassUsageMarker {

}

impl Default for ClassUsageMarker {
    fn default() -> Self {
        ClassUsageMarker {}
    }
}

impl ClassVisitor for ClassUsageMarker {
    fn visit_program_class(&self, program_clazz: ProgramClazz) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}