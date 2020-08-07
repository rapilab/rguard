use crate::classfile::clazz::Clazz;
use crate::classfile::visitor::member_visitor::MemberVisitor;
use crate::classfile::program_field::ProgramField;
use crate::classfile::program_method::ProgramMethod;

pub struct ProgramClass {
    fields: Vec<ProgramField>,
    methods: Vec<ProgramMethod>
}

impl ProgramClass {
    pub fn fields_accept(&self, member_visitor: MemberVisitor) {

    }
    pub fn method_accept(&self, member_visitor: MemberVisitor) {

    }
}

impl Clazz for ProgramClass {
    fn get_name() -> String {
        unimplemented!()
    }

    fn get_super_name() -> String {
        unimplemented!()
    }

    fn get_interface_name() -> String {
        unimplemented!()
    }
}