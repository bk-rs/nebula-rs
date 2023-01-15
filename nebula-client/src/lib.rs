//
pub mod v1;
pub mod v2;

//
pub trait Version {}

pub struct VersionV1;
impl Version for VersionV1 {}

pub struct VersionV2;
impl Version for VersionV2 {}
