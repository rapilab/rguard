use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use rguard_core::classfile::program_class::ProgramClass;
use rguard_core::classfile::program_field::ProgramField;
use rguard_core::classfile::program_member::ProgramMember;
use rguard_core::classfile::program_method::ProgramMethod;
use rguard_core::classfile::visitor::member_visitor::MemberVisitor;

pub struct UsedMemberFilter {
    usage_marker: SimpleUsageMarker,
    used_member_filter: Option<Box<dyn MemberVisitor>>,
    unused_member_visitor: Box<dyn MemberVisitor>,
}

impl UsedMemberFilter {
    pub fn new(
        usage_marker: SimpleUsageMarker,
        used_member_filter: Option<Box<dyn MemberVisitor>>,
        unused_member_visitor: Box<dyn MemberVisitor>,
    ) -> UsedMemberFilter {
        UsedMemberFilter {
            usage_marker,
            used_member_filter,
            unused_member_visitor,
        }
    }
}

impl MemberVisitor for UsedMemberFilter {
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
