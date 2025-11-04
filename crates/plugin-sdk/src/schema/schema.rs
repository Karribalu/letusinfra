use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

// Do we need Set in this schema?
#[derive(Debug, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "string")]
    TypeString,
    #[serde(rename = "int")]
    TypeInt,
    #[serde(rename = "float")]
    TypeFloat,
    #[serde(rename = "bool")]
    TypeBool,
    #[serde(rename = "list")]
    TypeList,
    #[serde(rename = "object")]
    TypeObject,
}
type SchemaDefaultFn = Box<dyn Fn() -> Option<SchemaElem>>;
type SchemaValidateFn = Box<dyn Fn(&SchemaElem) -> bool>;
type SchemaStateFn = Box<dyn Fn() -> String>;
#[derive(Serialize, Deserialize)]
pub struct Schema {
    /// The typing type must be one of the following:
    /// - `TypeString` - string
    /// - `TypeInt` - i64
    /// - `TypeFloat` - f64
    /// - `TypeBool` - bool
    /// - `TypeList` - vec
    /// - `TypeObject` - map
    /// If
    value_type: ValueType,

    /// The value of the resource schema
    elem: BTreeMap<String, SchemaElem>,

    /// `schema_version` is the version of the resource's schema definition.
    /// This field is None when the resource is not manager
    schema_version: Option<u64>,

    /// Minimum number of items in the typing type of array
    min_items: Option<u64>,

    /// Maximum number of items in the typing type of array
    max_items: Option<u64>,

    /// [`default`] indicates a value to set if this attribute is not set in the configuration
    /// `default` cannot be used with [`default_fn`] or [`required`].
    /// default is only supported if the value_type is String, Int, Float, Bool
    default: Option<SchemaElem>,

    /// Validation function to check if the typing is valid
    #[serde(skip)]
    validate_fn: Option<SchemaValidateFn>,

    /// Default typing for the field when not provided
    ///
    /// TODO: Do we need error support here?
    #[serde(skip)]
    default_fn: Option<SchemaDefaultFn>,

    /// A human-readable description of the attribute, Which will be used for documentation
    description: Option<String>,

    /// [`state_fn`] is a function called to change the value of this before
    /// storing it in the state (and likewise before comparing for diffs).
    /// The use for this is, for example, with large strings, you may want
    /// to simply store the hash of it.
    #[serde(skip)]
    state_fn: Option<SchemaStateFn>,

    /// [`conflicts_with`] is a list of attributes that cannot be set at the same time.
    /// This implements validation logic declaratively withing the schema and can trigger earlier in Yamlet operations
    ///
    /// Only absolute attribute paths, Ones starting with the top level attribute names, are supported,
    /// For TypeList (if [`max_items`] is greater than 1), TypeMap, or TypeSet
    /// attributes.
    /// To reference an attribute under a single configuration block
    /// (`TypeList` MaxItems of 1), the syntax is
    /// "parent_block_name.0.child_attribute_name".
    conflicts_with: Option<Vec<String>>,

    /// [`exactly_one_of`] is a list of attributes where exactly one must be set.
    /// It will return an error if none or more than one are set.
    /// This implements validation logic declaratively within the schema and can trigger earlier in Yamlet operations
    ///
    exactly_one_of: Option<Vec<String>>,

    /// [`atleast_one_of`] is a list of attributes where at least one must be set.
    /// It will return an error if none are set.
    /// This implements validation logic declaratively within the schema and can trigger earlier in Yamlet operations
    ///
    atleast_one_of: Option<Vec<String>>,

    /// [`required_with`] is a list of attributes where if this attribute is set,
    /// the listed attributes must also be set.
    /// This implements validation logic declaratively within the schema and can trigger earlier in Yamlet operations
    ///
    required_with: Option<Vec<String>>,

    /// [`sensitive`] marks the attribute as sensitive, which will prevent it from being displayed
    sensitive: bool,

    /// [`optional`] marks the attribute as optional, which means it does not need to be set in configuration
    optional: bool,

    /// [`required`] marks the attribute as required, which means it must be set in configuration
    required: bool,

    /// [`computed`] marks the attribute as computed, which means it is set by the provider
    computed: bool,

    /// [`force_new`] marks the attribute as force_new, which means changes to this attribute will require resource recreation
    force_new: bool,
}

#[derive(Serialize, Deserialize)]
pub enum SchemaElem {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Schema>),
    Object(BTreeMap<String, Schema>),
}

pub struct SchemaBuilder {
    schema: Schema,
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self {
            schema: Schema {
                value_type: ValueType::TypeObject,
                elem: BTreeMap::new(),
                schema_version: None,
                min_items: None,
                max_items: None,
                default: None,
                validate_fn: None,
                default_fn: None,
                description: None,
                state_fn: None,
                conflicts_with: None,
                exactly_one_of: None,
                atleast_one_of: None,
                required_with: None,
                sensitive: false,
                optional: false,
                required: false,
                computed: false,
                force_new: false,
            },
        }
    }

    pub fn value_type(mut self, vt: ValueType) -> Self {
        self.schema.value_type = vt;
        self
    }

    pub fn elem(mut self, elem: BTreeMap<String, SchemaElem>) -> Self {
        self.schema.elem = elem;
        self
    }

    pub fn schema_version(mut self, version: u64) -> Self {
        self.schema.schema_version = Some(version);
        self
    }

    pub fn min_items(mut self, min: u64) -> Self {
        self.schema.min_items = Some(min);
        self
    }

    pub fn max_items(mut self, max: u64) -> Self {
        self.schema.max_items = Some(max);
        self
    }

    pub fn default(mut self, d: SchemaElem) -> Self {
        self.schema.default = Some(d);
        self
    }

    pub fn validate_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&SchemaElem) -> bool + 'static,
    {
        self.schema.validate_fn = Some(Box::new(f));
        self
    }

    pub fn default_fn<F>(mut self, f: F) -> Self
    where
        F: Fn() -> Option<SchemaElem> + 'static,
    {
        self.schema.default_fn = Some(Box::new(f));
        self
    }

    pub fn description(mut self, desc: String) -> Self {
        self.schema.description = Some(desc);
        self
    }

    pub fn state_fn<F>(mut self, f: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        self.schema.state_fn = Some(Box::new(f));
        self
    }

    pub fn conflicts_with(mut self, conflicts: Vec<String>) -> Self {
        self.schema.conflicts_with = Some(conflicts);
        self
    }

    pub fn exactly_one_of(mut self, exactly_one: Vec<String>) -> Self {
        self.schema.exactly_one_of = Some(exactly_one);
        self
    }

    pub fn atleast_one_of(mut self, atleast_one: Vec<String>) -> Self {
        self.schema.atleast_one_of = Some(atleast_one);
        self
    }

    pub fn required_with(mut self, required: Vec<String>) -> Self {
        self.schema.required_with = Some(required);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.schema.sensitive = sensitive;
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.schema.optional = optional;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.schema.required = required;
        self
    }

    pub fn computed(mut self, computed: bool) -> Self {
        self.schema.computed = computed;
        self
    }

    pub fn force_new(mut self, force_new: bool) -> Self {
        self.schema.force_new = force_new;
        self
    }

    pub fn build(self) -> Schema {
        self.schema
    }
}

impl Schema {
    pub fn default_value(&self) -> Option<SchemaElem> {
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
            .field("elem_len", &self.elem.len())
            .field("schema_version", &self.schema_version)
            .field("min_items", &self.min_items)
            .field("max_items", &self.max_items)
            .field("has_default", &self.default.is_some())
            .field("has_validate_fn", &self.validate_fn.as_ref().map(|_| true))
            .field("has_default_fn", &self.default_fn.as_ref().map(|_| true))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lambda = |elem: &SchemaElem| {
            // Basic predicate that accepts only string default values longer than 0
            match elem {
                SchemaElem::String(s) => !s.is_empty(),
                _ => true,
            }
        };

        let schema = SchemaBuilder::new()
            .value_type(ValueType::TypeString)
            .schema_version(1)
            .min_items(1)
            .max_items(10)
            .default(SchemaElem::String("instance_type".to_string()))
            .validate_fn(lambda)
            .build();

        // Exercise default_value path as well
        let _ = schema.default_value();

        println!("Schema is {:?}", schema);
    }
}
