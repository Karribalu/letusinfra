use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
/// [`InstanceDiff`] is the diff of a resource between one state and another
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InstanceDiff {
    pub attributes: HashMap<String, String>,
    pub diff: BTreeMap<String, ResourceAttrDiff>,
    pub destroy: bool,
}

/// [`ResourceAttrDiff`] is the diff of a single attribute of a resource between one state and another
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceAttrDiff {
    /// OLD value of the attribute
    pub old: String,
    /// NEW value of the attribute
    pub new: String,
    /// True if the attribute is new
    pub new_computed: bool,
    /// True if the attribute is being removed
    pub new_removed: bool,
    /// True if the change requires a new resource
    pub requires_new: bool,
    /// True, if the attribute is sensitive, The UI should hide the value
    pub sensitive: bool,
    /// Type of the attribute, Whether it is provided by the user or computed by the provider
    pub diff_attr_type: DiffType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DiffType {
    Provided,
    Computed,
}
