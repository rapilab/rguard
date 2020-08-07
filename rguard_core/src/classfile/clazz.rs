pub trait Clazz: Sized  {
    fn get_name(&self) -> String;
    fn get_super_name(&self) -> String;
    fn get_interface_name(&self) -> String;
}


