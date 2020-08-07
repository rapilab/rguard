use crate::classfile::class_pool::ClassPool;
use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::program_clazz::ProgramClazz;
use crate::classfile::library_clazz::LibraryClazz;

pub struct ClassPoolFiller {
    pub class_pool: ClassPool
}

impl ClassPoolFiller {
    pub fn new(class_pool: ClassPool) -> ClassPoolFiller {
        ClassPoolFiller { class_pool }
    }
}

impl ClassVisitor for ClassPoolFiller {
    fn visit_program_class(&self, program_clazz: ProgramClazz) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}