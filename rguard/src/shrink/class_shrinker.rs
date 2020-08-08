use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use crate::shrink::used_member_filter::UsedMemberFilter;
use rguard_core::classfile::attribute::attribute::Attribute;
use rguard_core::classfile::attribute::attribute_visitor::AttributeVisitor;
use rguard_core::classfile::clazz::Clazz;
use rguard_core::classfile::library_clazz::LibraryClazz;
use rguard_core::classfile::program_class::ProgramClass;
use rguard_core::classfile::program_field::ProgramField;
use rguard_core::classfile::program_member::ProgramMember;
use rguard_core::classfile::program_method::ProgramMethod;
use rguard_core::classfile::visitor::all_attribute_visitor::AllAttributeVisitor;
use rguard_core::classfile::visitor::class_visitor::ClassVisitor;
use rguard_core::classfile::visitor::member_access_filter::MemberAccessFilter;
use rguard_core::classfile::visitor::member_visitor::MemberVisitor;

pub struct ClassShrinker {
    pub usage_marker: SimpleUsageMarker,
}

impl ClassShrinker {
    pub fn new(usage_marker: SimpleUsageMarker) -> ClassShrinker {
        ClassShrinker { usage_marker }
    }
}

pub struct MyNestMemberShrinker {}

impl MyNestMemberShrinker {
    pub fn new() -> MyNestMemberShrinker {
        MyNestMemberShrinker {}
    }
}

impl AttributeVisitor for MyNestMemberShrinker {
    fn visit_any_attribute(&self, clazz: Box<dyn Clazz>, attribute: Attribute) {
        unimplemented!()
    }
}

impl MemberVisitor for ClassShrinker {
    fn visit_program_member(self, program_class: ProgramClass, program_member: ProgramMember) {}
    fn visit_program_field(self, program_class: ProgramClass, program_field: ProgramField) {}
    fn visit_program_method(self, program_class: ProgramClass, program_method: ProgramMethod) {}
}

impl ClassVisitor for ClassShrinker {
    fn visit_program_class(&self, program_clazz: ProgramClass) {
        program_clazz.fields_accept(Box::from(UsedMemberFilter::new(
            self.usage_marker,
            None,
            Box::from(MemberAccessFilter::new()),
        )));
        program_clazz.method_accept(Box::from(MemberAccessFilter::new()));

        program_clazz.attributes_accept(Box::from(MyNestMemberShrinker::new()));

        // program_clazz.fields_accept(Box::from(self));
        // program_clazz.method_accept(Box::from(self));
        // program_clazz.attributes_accept(Box::from(self));

        program_clazz.fields_accept(Box::from(AllAttributeVisitor::new()));
        program_clazz.method_accept(Box::from(AllAttributeVisitor::new()));
        program_clazz.attributes_accept(Box::from(AllAttributeVisitor::new()));
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}
