use crate::classfile::class_pool::ClassPool;
use crate::classfile::library_clazz::LibraryClazz;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::clazz::Clazz;

pub struct ClassPoolFiller {
    pub class_pool: ClassPool,
}

impl ClassPoolFiller {
    pub fn new(class_pool: ClassPool) -> ClassPoolFiller {
        ClassPoolFiller { class_pool }
    }
}

impl ClassVisitor for ClassPoolFiller {
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
