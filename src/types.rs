// Type system for Oak Language
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    // Unsigned integers
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    
    // Signed integers
    Int8,
    Int16,
    Int32,
    Int64,
    
    // Positive integers (zero and above)
    Posint8,
    Posint16,
    Posint32,
    Posint64,
    
    // Floating point
    Float32,
    Float64,
    
    // Boolean
    Bool,
    
    // String types
    Str,      // Fixed length string
    Text,     // Large dynamic string
    
    // Void
    Void,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrimitiveType::Uint8 => write!(f, "u8"),
            PrimitiveType::Uint16 => write!(f, "u16"),
            PrimitiveType::Uint32 => write!(f, "u32"),
            PrimitiveType::Uint64 => write!(f, "u64"),
            PrimitiveType::Int8 => write!(f, "i8"),
            PrimitiveType::Int16 => write!(f, "i16"),
            PrimitiveType::Int32 => write!(f, "i32"),
            PrimitiveType::Int64 => write!(f, "i64"),
            PrimitiveType::Posint8 => write!(f, "posint8"),
            PrimitiveType::Posint16 => write!(f, "posint16"),
            PrimitiveType::Posint32 => write!(f, "posint32"),
            PrimitiveType::Posint64 => write!(f, "posint64"),
            PrimitiveType::Float32 => write!(f, "f32"),
            PrimitiveType::Float64 => write!(f, "f64"),
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Str => write!(f, "str"),
            PrimitiveType::Text => write!(f, "Text"),
            PrimitiveType::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OakType {
    Primitive(PrimitiveType),
    Array(Box<OakType>, Option<usize>), // array type and optional size
    Pointer(Box<OakType>),               // pointer to type
    Reference(Box<OakType>),             // reference to type
    Function {
        params: Vec<OakType>,
        return_type: Box<OakType>,
    },
    Struct(String),                      // struct name
    Enum(String),                        // enum name
    Generic(String, Vec<OakType>),       // generic type with parameters
    Inferred,                            // type to be inferred
}

impl fmt::Display for OakType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OakType::Primitive(prim) => write!(f, "{}", prim),
            OakType::Array(ty, size) => {
                if let Some(s) = size {
                    write!(f, "[{}; {}]", ty, s)
                } else {
                    write!(f, "[{}]", ty)
                }
            }
            OakType::Pointer(ty) => write!(f, "*{}", ty),
            OakType::Reference(ty) => write!(f, "&{}", ty),
            OakType::Function { params, return_type } => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            OakType::Struct(name) => write!(f, "struct {}", name),
            OakType::Enum(name) => write!(f, "enum {}", name),
            OakType::Generic(name, params) => {
                write!(f, "{}<", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ">")
            }
            OakType::Inferred => write!(f, "inferred"),
        }
    }
}

impl OakType {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            OakType::Primitive(
                PrimitiveType::Int8
                    | PrimitiveType::Int16
                    | PrimitiveType::Int32
                    | PrimitiveType::Int64
                    | PrimitiveType::Uint8
                    | PrimitiveType::Uint16
                    | PrimitiveType::Uint32
                    | PrimitiveType::Uint64
                    | PrimitiveType::Posint8
                    | PrimitiveType::Posint16
                    | PrimitiveType::Posint32
                    | PrimitiveType::Posint64
                    | PrimitiveType::Float32
                    | PrimitiveType::Float64
            )
        )
    }
    
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            OakType::Primitive(
                PrimitiveType::Int8
                    | PrimitiveType::Int16
                    | PrimitiveType::Int32
                    | PrimitiveType::Int64
                    | PrimitiveType::Uint8
                    | PrimitiveType::Uint16
                    | PrimitiveType::Uint32
                    | PrimitiveType::Uint64
                    | PrimitiveType::Posint8
                    | PrimitiveType::Posint16
                    | PrimitiveType::Posint32
                    | PrimitiveType::Posint64
            )
        )
    }
    
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            OakType::Primitive(PrimitiveType::Float32 | PrimitiveType::Float64)
        )
    }
    
    pub fn can_cast_to(&self, target: &OakType) -> bool {
        // Same types can always be "cast"
        if self == target {
            return true;
        }
        
        // Numeric types can be cast to other numeric types
        if self.is_numeric() && target.is_numeric() {
            return true;
        }
        
        // Pointer types can cast to void pointer
        if let OakType::Pointer(_) = self {
            if let OakType::Pointer(box OakType::Primitive(PrimitiveType::Void)) = target {
                return true;
            }
        }
        
        false
    }
}

#[derive(Debug, Clone)]
pub struct TypeContext {
    variables: HashMap<String, OakType>,
    functions: HashMap<String, (Vec<OakType>, OakType)>, // name -> (params, return_type)
    structs: HashMap<String, Vec<(String, OakType)>>,    // name -> fields
    enums: HashMap<String, Vec<String>>,                 // name -> variants
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
        }
    }
    
    pub fn declare_variable(&mut self, name: String, ty: OakType) {
        self.variables.insert(name, ty);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&OakType> {
        self.variables.get(name)
    }
    
    pub fn declare_function(&mut self, name: String, params: Vec<OakType>, return_type: OakType) {
        self.functions.insert(name, (params, return_type));
    }
    
    pub fn get_function(&self, name: &str) -> Option<&(Vec<OakType>, OakType)> {
        self.functions.get(name)
    }
    
    pub fn declare_struct(&mut self, name: String, fields: Vec<(String, OakType)>) {
        self.structs.insert(name, fields);
    }
    
    pub fn get_struct(&self, name: &str) -> Option<&Vec<(String, OakType)>> {
        self.structs.get(name)
    }
}
