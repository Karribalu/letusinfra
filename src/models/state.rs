use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub version: String,
    pub modulepack: String,
    pub outputs: Output,
    pub components: Vec<ComponentState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentMode {
    BYO,
    Managed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentState {
    pub mode: ComponentMode,
    pub name: String,
    #[serde(rename = "type")]
    pub component_type: String,
    pub provider: String,
    pub instances: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    pub value: serde_json::Value,
    pub output_type: OutputType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Datatype {
    String,
    Number,
    Object,
    Map,
    List,
    Bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputType {
    pub datatype: Datatype,
    pub value: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_component_mode_serialization() {
        let byo = ComponentMode::BYO;
        let managed = ComponentMode::Managed;

        assert_eq!(serde_json::to_string(&byo).unwrap(), r#""byo""#);
        assert_eq!(serde_json::to_string(&managed).unwrap(), r#""managed""#);
    }

    #[test]
    fn test_component_mode_deserialization() {
        let byo: ComponentMode = serde_json::from_str(r#""byo""#).unwrap();
        let managed: ComponentMode = serde_json::from_str(r#""managed""#).unwrap();

        assert_eq!(byo, ComponentMode::BYO);
        assert_eq!(managed, ComponentMode::Managed);
    }

    #[test]
    fn test_datatype_serialization() {
        assert_eq!(
            serde_json::to_string(&Datatype::String).unwrap(),
            r#""string""#
        );
        assert_eq!(
            serde_json::to_string(&Datatype::Number).unwrap(),
            r#""number""#
        );
        assert_eq!(
            serde_json::to_string(&Datatype::Object).unwrap(),
            r#""object""#
        );
        assert_eq!(serde_json::to_string(&Datatype::Map).unwrap(), r#""map""#);
        assert_eq!(serde_json::to_string(&Datatype::List).unwrap(), r#""list""#);
        assert_eq!(serde_json::to_string(&Datatype::Bool).unwrap(), r#""bool""#);
    }

    #[test]
    fn test_output_type_serialization() {
        let output_type = OutputType {
            datatype: Datatype::String,
            value: json!("test"),
        };

        let serialized = serde_json::to_value(&output_type).unwrap();
        assert_eq!(serialized["datatype"], "string");
        assert_eq!(serialized["value"], "test");
    }

    #[test]
    fn test_output_serialization() {
        let output = Output {
            value: json!({"key": "value"}),
            output_type: OutputType {
                datatype: Datatype::Object,
                value: json!({}),
            },
        };

        let serialized = serde_json::to_value(&output).unwrap();
        assert!(serialized["value"].is_object());
        assert_eq!(serialized["output_type"]["datatype"], "object");
    }

    #[test]
    fn test_component_state_serialization() {
        let component = ComponentState {
            mode: ComponentMode::Managed,
            name: "test-component".to_string(),
            component_type: "database".to_string(),
            provider: "aws".to_string(),
            instances: vec![json!({"id": "instance-1"})],
        };

        let serialized = serde_json::to_value(&component).unwrap();
        assert_eq!(serialized["mode"], "managed");
        assert_eq!(serialized["name"], "test-component");
        assert_eq!(serialized["type"], "database");
        assert_eq!(serialized["provider"], "aws");
        assert!(serialized["instances"].is_array());
    }

    #[test]
    fn test_component_state_deserialization() {
        let json_str = r#"{
            "mode": "byo",
            "name": "my-component",
            "type": "storage",
            "provider": "gcp",
            "instances": [{"id": "inst-1"}]
        }"#;

        let component: ComponentState = serde_json::from_str(json_str).unwrap();
        assert_eq!(component.mode, ComponentMode::BYO);
        assert_eq!(component.name, "my-component");
        assert_eq!(component.component_type, "storage");
        assert_eq!(component.provider, "gcp");
        assert_eq!(component.instances.len(), 1);
    }

    #[test]
    fn test_state_complete_serialization() {
        let state = State {
            version: "1.0.0".to_string(),
            modulepack: "test-pack".to_string(),
            outputs: Output {
                value: json!({"endpoint": "http://example.com"}),
                output_type: OutputType {
                    datatype: Datatype::Object,
                    value: json!({}),
                },
            },
            components: vec![ComponentState {
                mode: ComponentMode::Managed,
                name: "db".to_string(),
                component_type: "postgres".to_string(),
                provider: "aws".to_string(),
                instances: vec![json!({"id": "db-1"})],
            }],
        };

        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: State = serde_json::from_str(&serialized).unwrap();

        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_state_deserialization() {
        let json_str = r#"{
            "version": "2.0.0",
            "modulepack": "prod-pack",
            "outputs": {
                "value": {"result": "success"},
                "output_type": {
                    "datatype": "object",
                    "value": {}
                }
            },
            "components": [
                {
                    "mode": "byo",
                    "name": "cache",
                    "type": "redis",
                    "provider": "azure",
                    "instances": []
                }
            ]
        }"#;

        let state: State = serde_json::from_str(json_str).unwrap();
        assert_eq!(state.version, "2.0.0");
        assert_eq!(state.modulepack, "prod-pack");
        assert_eq!(state.components.len(), 1);
        assert_eq!(state.components[0].mode, ComponentMode::BYO);
    }

    #[test]
    fn test_empty_components() {
        let state = State {
            version: "1.0.0".to_string(),
            modulepack: "empty-pack".to_string(),
            outputs: Output {
                value: json!(null),
                output_type: OutputType {
                    datatype: Datatype::Object,
                    value: json!({}),
                },
            },
            components: vec![],
        };

        assert_eq!(state.components.len(), 0);
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: State = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }
}
