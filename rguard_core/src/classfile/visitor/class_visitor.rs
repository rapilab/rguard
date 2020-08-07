use crate::classfile::program_clazz::ProgramClazz;
use crate::classfile::library_clazz::LibraryClazz;

pub trait ClassVisitor {
    // fn visit_any_class(clazz: dyn Clazz);
    fn visit_program_class(&self, program_clazz: ProgramClazz);
    fn visit_library_class(&self, library_clazz: LibraryClazz);
}
