use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::program_clazz::ProgramClazz;
use crate::classfile::library_clazz::LibraryClazz;

pub struct MultiClassVisitor {

}

impl ClassVisitor for MultiClassVisitor {
    fn visit_program_class(&self, program_clazz: ProgramClazz) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
