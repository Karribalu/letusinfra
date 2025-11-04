use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// [`InstanceState`] is used to track the unique state information of a resource
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstanceState {
    /// A unique `id` for the resource. This is opaque to Yamlet.
    /// and is only meant as a lookup mechanism for the providers
    id: String,

    /// `attributes` is used to store the resource attributes
    attributes: HashMap<String, String>,

    /// `identity` is used to store the resource identity information
    identity: HashMap<String, String>,
}

impl InstanceState {
    pub fn new(
        id: String,
        attributes: HashMap<String, String>,
        identity: HashMap<String, String>,
    ) -> Self {
        InstanceState {
            id,
            attributes,
            identity,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn identity(&self) -> &HashMap<String, String> {
        &self.identity
    }
}
