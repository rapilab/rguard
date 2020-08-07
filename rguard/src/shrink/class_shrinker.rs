use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use rguard_core::classfile::visitor::class_visitor::ClassVisitor;
use rguard_core::classfile::program_class::ProgramClass;
use rguard_core::classfile::library_clazz::LibraryClazz;
use rguard_core::classfile::visitor::member_access_filter::MemberAccessFilter;
use crate::shrink::used_member_filter::UsedMemberFilter;

pub struct ClassShrinker {
    pub usage_marker: SimpleUsageMarker
}

impl ClassShrinker {
    pub fn new(usage_marker: SimpleUsageMarker) -> ClassShrinker {
        ClassShrinker { usage_marker }
    }
}

pub struct MyNestMemberShrinker {

}

impl MyNestMemberShrinker {}


impl ClassVisitor for ClassShrinker {
    fn visit_program_class(&self, program_clazz: ProgramClass) {
        program_clazz.fields_accept(
            Box::from(UsedMemberFilter::new(
                self.usage_marker,
                None,
                Box::from(MemberAccessFilter::new())
            ))
        );
        program_clazz.method_accept(
            Box::from(MemberAccessFilter::new())
        );
    }

    fn visit_library_class(&self, library_clazz: LibraryClazz) {
        unimplemented!()
    }
}