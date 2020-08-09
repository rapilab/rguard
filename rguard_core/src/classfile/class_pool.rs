use crate::classfile::visitor::class_visitor::ClassVisitor;
use std::collections::HashMap;
use crate::classfile::clazz::Clazz;

#[derive(Clone)]
pub struct ClassPool {
    classes: HashMap<String, Box<dyn Clazz>>
}

impl Default for ClassPool {
    fn default() -> Self {
        ClassPool { classes: Default::default() }
    }
}

impl ClassPool {
    pub fn add_class(&mut self, clazz: Box<dyn Clazz>) {
        self.classes.insert(clazz.get_name(), clazz);
    }
    pub fn classes_accept(&self, cleaner: Box<dyn ClassVisitor>) {}
}
