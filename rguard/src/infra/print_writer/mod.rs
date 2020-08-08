pub struct PrintWriter {
    // out: Box<dyn Fn(&mut T)>
    pub out: Box<()>,
}

impl PrintWriter {
    pub fn new() -> PrintWriter {
        PrintWriter { out: Box::new(()) }
    }
    // pub fn new_fn<F>(self, mut f: F) -> A where F: FnMut(A)  {
    //     let mut writer = PrintWriter {
    //         out: Box::new(())
    //     };
    //
    //     writer.out = f;
    //     writer
    // }
}
