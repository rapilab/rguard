use crate::classfile::attribute::attribute::Attribute;
use crate::classfile::clazz::Clazz;

pub trait AttributeVisitor {
    fn visit_any_attribute(&self, clazz: Box<dyn Clazz>, attribute: Attribute);
}
