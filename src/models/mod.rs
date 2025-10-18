use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfraConfig {
    pub version: String,
    pub kind: String,
    pub cloud: String,
    pub region: String,
    pub metadata: Metadata,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,
    pub name: String,
    #[serde(default)]
    pub properties: HashMap<String, serde_yaml::Value>,
    #[serde(rename = "dependsOn", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<Dependency>>,
    #[serde(rename = "connectsTo", skip_serializing_if = "Option::is_none")]
    pub connects_to: Option<Vec<Dependency>>,
}

impl Hash for Component {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.component_type.hash(state);
        self.name.hash(state);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Dependency {
    #[serde(rename = "type")]
    pub dep_type: String,
    pub name: String,
}

impl InfraConfig {
    /// Parse YAML content into InfraConfig
    pub fn from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    /// Convert InfraConfig to YAML string
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl Component {
    /// Get a property value by key
    pub fn get_property(&self, key: &str) -> Option<&serde_yaml::Value> {
        self.properties.get(key)
    }

    /// Get a property as a string
    pub fn get_property_as_string(&self, key: &str) -> Option<String> {
        self.get_property(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Get a property as a boolean
    pub fn get_property_as_bool(&self, key: &str) -> Option<bool> {
        self.get_property(key).and_then(|v| v.as_bool())
    }

    /// Get a property as an integer
    pub fn get_property_as_i64(&self, key: &str) -> Option<i64> {
        self.get_property(key).and_then(|v| v.as_i64())
    }

    /// Get a property as a float
    pub fn get_property_as_f64(&self, key: &str) -> Option<f64> {
        self.get_property(key).and_then(|v| v.as_f64())
    }

    /// Get a property as an array of strings
    pub fn get_property_as_string_array(&self, key: &str) -> Option<Vec<String>> {
        self.get_property(key).and_then(|v| {
            v.as_sequence().map(|seq| {
                seq.iter()
                    .filter_map(|item| item.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("Invalid component: {0}")]
    InvalidComponent(String),
    #[error("Missing mandatory property '{0}' in component '{1}'")]
    MissingProperty(String, String),
    #[error("Invalid property type for '{0}' in component '{1}' : expected {2}, found {3}")]
    InvalidPropertyType(String, String, String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_infra_config() {
        let yaml_content = r#"
version: v1
kind: Infra
cloud: AWS
region: us-west-2
metadata:
  name: sample
components:
  - type: VPC
    name: sample-vpc
    properties:
      cidr: 10.0.0.0/16
  - type: ECSCluster
    name: sample-cluster
    properties:
      cluster_name: sample-cluster
      size: 3
    dependsOn:
      - type: VPC
        name: sample-vpc
"#;

        let config = InfraConfig::from_yaml(yaml_content).unwrap();
        assert_eq!(config.version, "v1");
        assert_eq!(config.kind, "Infra");
        assert_eq!(config.metadata.name, "sample");
        assert_eq!(config.components.len(), 2);
        assert_eq!(config.components[0].component_type, "VPC");
        assert_eq!(config.components[0].name, "sample-vpc");
        assert!(config.components[1].depends_on.is_some());
    }
}
