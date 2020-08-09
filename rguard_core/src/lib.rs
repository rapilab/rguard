pub mod classfile;
pub mod resources;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Foo {
        common: Rc<RefCell<i32>>,
        bar: Bar,
    }

    #[derive(Debug)]
    struct Bar {
        common: Rc<RefCell<i32>>,
    }

    impl Bar {
        pub fn new(common: Rc<RefCell<i32>>) -> Bar {
            Bar {
                common: common,
            }
        }
    }

    impl Foo {
        pub fn new() -> Foo {
            let cmn = Rc::new(RefCell::new(42));
            Foo { common: cmn.clone(), bar: Bar::new(cmn) }
        }
    }

    #[test]
    fn should_do_something() {
        let foo  = Foo::new();
        println!("{:?}", foo);
        *foo.common.borrow_mut() = 6 * 9;
        println!("{:?}", foo);
    }

}
