use crate::classfile::attribute::attribute::Attribute;
use crate::classfile::attribute::attribute_visitor::AttributeVisitor;
use crate::classfile::clazz::Clazz;
use crate::classfile::library_clazz::LibraryClazz;
use crate::classfile::program_class::ProgramClass;
use crate::classfile::program_field::ProgramField;
use crate::classfile::program_member::ProgramMember;
use crate::classfile::program_method::ProgramMethod;
use crate::classfile::visitor::class_visitor::ClassVisitor;
use crate::classfile::visitor::member_visitor::MemberVisitor;

pub struct AllAttributeVisitor {}

impl AllAttributeVisitor {
    pub fn new() -> AllAttributeVisitor {
        AllAttributeVisitor {}
    }
}

impl AttributeVisitor for AllAttributeVisitor {
    fn visit_any_attribute(&self, clazz: Box<dyn Clazz>, attribute: Attribute) {}
}

impl MemberVisitor for AllAttributeVisitor {
    fn visit_program_member(self, program_class: ProgramClass, program_member: ProgramMember) {}
    fn visit_program_field(self, program_class: ProgramClass, program_field: ProgramField) {}
    fn visit_program_method(self, program_class: ProgramClass, program_method: ProgramMethod) {}
}

impl ClassVisitor for AllAttributeVisitor {
    fn visit_any_class(&self, clazz: Box<dyn Clazz>) {}
    fn visit_program_class(&self, program_clazz: ProgramClass) {}
    fn visit_library_class(&self, library_clazz: LibraryClazz) {}
}
