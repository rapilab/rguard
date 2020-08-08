use crate::classfile::class_pool::ClassPool;
use crate::classfile::library_clazz::LibraryClazz;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::visitor::class_visitor::ClassVisitor;

pub struct ClassPoolFiller {
    pub class_pool: ClassPool,
}

impl ClassPoolFiller {
    pub fn new(class_pool: ClassPool) -> ClassPoolFiller {
        ClassPoolFiller { class_pool }
    }
}

impl ClassVisitor for ClassPoolFiller {
    fn visit_program_class(&self, program_clazz: ProgramClass) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
