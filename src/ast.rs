// Define enums for types and attributes
#[derive(Debug, Clone)]
enum DataType {
    UUID,
    String,
    DateTime,
    Data,
    Int,
    Float,
    Boolean,
    Enum(String), // Stores the name of the enum
}

#[derive(Debug, Clone)]
enum Attribute {
    Queryable,
    Default(String), // Stores the default value as a string
    Optional,
}

// AST Nodes
#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub attributes: Vec<Attribute>,
    pub inner_type: Option<String>, // For array types or inner types of models/enums
}

#[derive(Debug, Clone)]
pub enum EnumValue {
    Variant(String),
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

