use rguard_core::classfile::library_clazz::LibraryClazz;
use rguard_core::classfile::program_class::ProgramClass;
use rguard_core::classfile::visitor::class_visitor::ClassVisitor;
use rguard_core::classfile::visitor::member_visitor::MemberVisitor;
use rguard_core::classfile::program_member::ProgramMember;
use rguard_core::classfile::program_method::ProgramMethod;
use rguard_core::classfile::program_field::ProgramField;
use rguard_core::classfile::attribute::attribute_visitor::AttributeVisitor;
use rguard_core::classfile::attribute::attribute::Attribute;
use rguard_core::classfile::clazz::Clazz;

pub struct ClassUsageMarker {}

impl Default for ClassUsageMarker {
    fn default() -> Self {
        ClassUsageMarker {}
    }
}

impl ClassVisitor for ClassUsageMarker {
    fn visit_program_class(&self, program_clazz: ProgramClass) {
        unimplemented!()
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}

impl MemberVisitor for ClassUsageMarker {
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

impl AttributeVisitor for ClassVisitor {
    fn visit_any_attribute(&self, clazz: Box<dyn Clazz>, attribute: Attribute) {
        unimplemented!()
    }
}