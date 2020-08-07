use crate::classfile::clazz::Clazz;
use crate::classfile::attribute::attribute::Attribute;

pub trait AttributeVisitor {
    // fn visit_any_attribute(&self, clazz: Box<dyn Clazz>, attribute: Attribute);
}
