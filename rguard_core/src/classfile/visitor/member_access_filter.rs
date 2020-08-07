use crate::classfile::visitor::member_visitor::MemberVisitor;
use crate::classfile::program_member::ProgramMember;
use crate::classfile::program_method::ProgramMethod;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::program_field::ProgramField;

pub struct MemberAccessFilter {

}

impl MemberAccessFilter {
    pub fn new() -> MemberAccessFilter {
        MemberAccessFilter {}
    }
}

impl MemberVisitor for MemberAccessFilter {
    fn visit_program_member(self, program_class: ProgramClass, program_member: ProgramMember) {
        unimplemented!()
    }

    fn visit_program_field(self, program_class: ProgramClass, program_field: ProgramField) {
        unimplemented!()
    }

    fn visit_program_method(self, program_class: ProgramClass, program_method: ProgramMethod) {
        unimplemented!()
    }
}