pub trait Clazz: ClazzClone {
    fn get_name(&self) -> String;
    fn get_super_name(&self) -> String;
    fn get_interface_name(&self) -> String;
}

pub trait ClazzClone {
    fn clone_box(&self) -> Box<dyn Clazz>;
}

impl<T> ClazzClone for T
    where
        T: 'static + Clazz + Clone,
{
    fn clone_box(&self) -> Box<dyn Clazz> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Clazz> {
    fn clone(&self) -> Box<dyn Clazz> {
        self.clone_box()
    }
}
