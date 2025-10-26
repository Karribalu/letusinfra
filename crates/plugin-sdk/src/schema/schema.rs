use std::collections::HashMap;
use std::fmt;
// Do we need Set in this schema?
#[derive(Debug)]
pub enum ValueType {
    String(String),
    Number(u64),
    Boolean(bool),
    Array(Vec<ValueType>),
    Map(HashMap<String, ValueType>),
}
pub struct Schema {
    value_type: Option<ValueType>,
    /** Minimum number of items in the value type of array or map */
    min_items: Option<u64>,
    /** Maximum number of items in the value type of array or map */
    max_items: Option<u64>,
    validate_fn: Option<Box<dyn Fn(&ValueType) -> bool>>,
    /** Default value for the field when not provided*/
    // TODO: Do we need error support here?
    default_fn: Option<Box<dyn Fn() -> Option<ValueType>>>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            value_type: None,
            min_items: None,
            max_items: None,
            validate_fn: None,
            default_fn: None,
        }
    }

    pub fn default_value(&self) -> Option<ValueType> {
        if let Some(default_fn) = &self.default_fn {
            default_fn()
        } else {
            None
        }
    }
}

impl fmt::Debug for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Schema")
            .field("value_type", &self.value_type)
            .field("min_items", &self.min_items)
            .field("max_items", &self.max_items)
            .field("validate_fn", &"Fn(...)")
            .finish()
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lambda = |value: &ValueType| {
            println!("Value is {:?}", value);
            true
        };
        let schema = Schema {
            value_type: Option::from(ValueType::String("hello".to_string())),
            min_items: None,
            max_items: None,
            validate_fn: Some(Box::new(lambda)),
            default_fn: None,
        };

        let _ = &schema.validate_fn.as_ref().unwrap()(&schema.value_type.as_ref().unwrap());

        println!("Schema is {:?}", schema);
    }
}
