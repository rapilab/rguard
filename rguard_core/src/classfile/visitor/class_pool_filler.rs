use crate::classfile::class_pool::ClassPool;
use crate::classfile::library_clazz::LibraryClazz;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::clazz::Clazz;

pub struct ClassPoolFiller<'a> {
    pub class_pool: &'a ClassPool
}

impl<'a> ClassPoolFiller<'a> {
    pub fn new(class_pool: &'a mut ClassPool) -> ClassPoolFiller<'a> {
        ClassPoolFiller { class_pool: class_pool }
    }
}

impl<'a> ClassVisitor for ClassPoolFiller<'a> {
    fn visit_any_class(&mut self, clazz: Box<dyn Clazz>) {
        // self.class_pool.add_class(clazz);
    }

    fn visit_program_class(&mut self, program_clazz: ProgramClass) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
