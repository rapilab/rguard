use crate::classfile::clazz::Clazz;
use crate::classfile::program_clazz::ProgramClazz;
use crate::classfile::library_clazz::LibraryClazz;
use std::ops::Add;

pub trait ClassVisitor: Sized {
    // fn visit_any_class(clazz: dyn Clazz);
    fn visit_program_class(program_clazz: ProgramClazz);
    fn visit_library_class(library_clazz: LibraryClazz);
}
