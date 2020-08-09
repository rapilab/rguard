use crate::classfile::library_clazz::LibraryClazz;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::clazz::Clazz;

pub trait ClassVisitor {
    fn visit_any_class(&mut self, clazz: Box<dyn Clazz>);
    fn visit_program_class(&mut self, program_clazz: ProgramClass) {
        self.visit_any_class(Box::from(program_clazz))
    }
    fn visit_library_class(&self, library_clazz: LibraryClazz);
}
