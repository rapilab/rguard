use rguard::shrink::shrinker::Shrinker;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;

pub struct RGuard {
    program_class_pool: ClassPool,
    library_class_pool: ClassPool,
    resource_file_pool: ResourceFilePool,
}

impl Default for RGuard {
    fn default() -> Self {
        RGuard {
            program_class_pool: Default::default(),
            library_class_pool: Default::default(),
            resource_file_pool: Default::default()
        }
    }
}

impl RGuard {
    pub fn execute(&self) {
        self.shrink();
        self.optimize();
        self.obfuscate();
        self.preverify();
        self.write_output();
    }

    pub fn shrink(&self) {
        let shrinker = Shrinker::default();
        shrinker.execute();
    }

    pub fn optimize(&self) {}

    pub fn obfuscate(&self) {}

    pub fn preverify(&self) {}

    pub fn write_output(&self) {}
}
