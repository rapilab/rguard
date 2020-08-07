use crate::configuration::Configuration;
use rguard_core::classfile::class_pool::ClassPool;
use rguard_core::resources::resource_file_pool::ResourceFilePool;
use crate::shrink::simple_usage_marker::SimpleUsageMarker;
use crate::shrink::class_usage_marker::ClassUsageMarker;

pub struct UsageMarker {
    pub configuration: Configuration
}

impl UsageMarker {
    pub fn new(configuration: Configuration) -> UsageMarker {
        UsageMarker { configuration }
    }

    pub fn mark(&self,
                program_class_pool: ClassPool,
                library_class_pool: ClassPool,
                resource_file_pool: ResourceFilePool,
                simple_usage_marker: SimpleUsageMarker,
                class_usage_marker: ClassUsageMarker
    ) {

    }
}