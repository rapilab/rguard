use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use rguard_core::classfile::visitor::member_visitor::MemberVisitor;

pub struct UsedMemberFilter {
    usage_marker: SimpleUsageMarker,
    used_member_filter: Option<Box<dyn MemberVisitor>>,
    unused_member_visitor: Box<dyn MemberVisitor>
}

impl UsedMemberFilter {
    pub fn new() {

    }
}

