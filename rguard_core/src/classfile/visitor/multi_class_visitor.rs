use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::library_clazz::LibraryClazz;

pub struct MultiClassVisitor {
    pub visitors: Vec<Box<dyn ClassVisitor>>
}

impl MultiClassVisitor {
    pub fn new(visitors: Vec<Box<dyn ClassVisitor>>) -> MultiClassVisitor {
        MultiClassVisitor { visitors }
    }
}

impl ClassVisitor for MultiClassVisitor {
    fn visit_program_class(&self, program_clazz: ProgramClass) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
