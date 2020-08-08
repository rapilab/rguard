use crate::classfile::attribute::attribute_visitor::AttributeVisitor;
use crate::classfile::clazz::Clazz;
use crate::classfile::program_field::ProgramField;
use crate::classfile::program_method::ProgramMethod;
use crate::classfile::visitor::member_visitor::MemberVisitor;

pub struct ProgramClass {
    fields: Vec<ProgramField>,
    methods: Vec<ProgramMethod>,
}

impl ProgramClass {
    pub fn fields_accept(&self, member_visitor: Box<dyn MemberVisitor>) {}
    pub fn method_accept(&self, member_visitor: Box<dyn MemberVisitor>) {}
    pub fn attributes_accept(&self, attribute_visitor: Box<dyn AttributeVisitor>) {}
}

impl Clazz for ProgramClass {
    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn get_super_name(&self) -> String {
        unimplemented!()
    }

    fn get_interface_name(&self) -> String {
        unimplemented!()
    }
}
