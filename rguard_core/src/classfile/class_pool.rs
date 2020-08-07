use crate::classfile::visitor::class_visitor::ClassVisitor;

#[derive(Copy, Clone)]
pub struct ClassPool {}

impl Default for ClassPool {
    fn default() -> Self {
        ClassPool {}
    }
}
impl ClassPool {
    pub fn classes_accept(&self, cleaner: Box<dyn ClassVisitor>) {

    }
}
