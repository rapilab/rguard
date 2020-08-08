use crate::classfile::program_class::ProgramClass;
use crate::classfile::program_field::ProgramField;
use crate::classfile::program_member::ProgramMember;
use crate::classfile::program_method::ProgramMethod;

pub trait MemberVisitor {
    fn visit_program_member(self, program_class: ProgramClass, program_member: ProgramMember);
    fn visit_program_field(self, program_class: ProgramClass, program_field: ProgramField);
    fn visit_program_method(self, program_class: ProgramClass, program_method: ProgramMethod);
}
