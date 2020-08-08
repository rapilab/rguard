use crate::classfile::clazz::Clazz;

pub struct LibraryClazz {}

impl Clazz for LibraryClazz {
    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn get_super_name(&self) -> String {
        unimplemented!()
    }

    fn get_interface_name(&self) -> String {
        unimplemented!()
    }
}
