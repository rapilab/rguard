use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::program_clazz::ProgramClazz;
use crate::classfile::library_clazz::LibraryClazz;

#[derive(Copy, Clone)]
pub struct ClassCleaner {}

impl ClassCleaner {

}

impl Default for ClassCleaner {
    fn default() -> Self {
        ClassCleaner {}
    }
}


impl ClassVisitor for ClassCleaner {
    fn visit_program_class(&self, program_clazz: ProgramClazz) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}