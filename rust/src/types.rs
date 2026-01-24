/// Type system for graph-based type inference
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Type {
    /// Instance type: String, Integer, etc.
    Instance { class_name: String },
    /// Generic instance type: Array[Integer], Hash[String, Integer], etc.
    Generic {
        class_name: String,
        type_args: Vec<Type>,
    },
    /// Singleton type: for class methods
    Singleton { class_name: String },
    /// nil type
    Nil,
    /// Union type: sum of multiple types
    Union(Vec<Type>),
    /// Bottom type: no type information
    Bot,
}

impl Type {
    /// Convert type to string representation
    pub fn show(&self) -> String {
        match self {
            Type::Instance { class_name } => class_name.clone(),
            Type::Generic {
                class_name,
                type_args,
            } => {
                let args: Vec<_> = type_args.iter().map(|t| t.show()).collect();
                format!("{}[{}]", class_name, args.join(", "))
            }
            Type::Singleton { class_name } => format!("singleton({})", class_name),
            Type::Nil => "nil".to_string(),
            Type::Union(types) => {
                let names: Vec<_> = types.iter().map(|t| t.show()).collect();
                names.join(" | ")
            }
            Type::Bot => "untyped".to_string(),
        }
    }

    /// Get the base class name (without type arguments)
    pub fn base_class_name(&self) -> Option<&str> {
        match self {
            Type::Instance { class_name } => Some(class_name),
            Type::Generic { class_name, .. } => Some(class_name),
            Type::Singleton { class_name } => Some(class_name),
            _ => None,
        }
    }

    /// Get type arguments for generic types
    pub fn type_args(&self) -> Option<&[Type]> {
        match self {
            Type::Generic { type_args, .. } => Some(type_args),
            _ => None,
        }
    }

    /// Convenience constructors
    pub fn string() -> Self {
        Type::Instance {
            class_name: "String".to_string(),
        }
    }

    pub fn integer() -> Self {
        Type::Instance {
            class_name: "Integer".to_string(),
        }
    }

    pub fn array() -> Self {
        Type::Instance {
            class_name: "Array".to_string(),
        }
    }

    pub fn hash() -> Self {
        Type::Instance {
            class_name: "Hash".to_string(),
        }
    }

    /// Create a generic Array type: Array[element_type]
    pub fn array_of(element_type: Type) -> Self {
        Type::Generic {
            class_name: "Array".to_string(),
            type_args: vec![element_type],
        }
    }

    /// Create a generic Hash type: Hash[key_type, value_type]
    pub fn hash_of(key_type: Type, value_type: Type) -> Self {
        Type::Generic {
            class_name: "Hash".to_string(),
            type_args: vec![key_type, value_type],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_show() {
        assert_eq!(Type::string().show(), "String");
        assert_eq!(Type::integer().show(), "Integer");
        assert_eq!(Type::Nil.show(), "nil");
        assert_eq!(Type::Bot.show(), "untyped");
    }

    #[test]
    fn test_type_union() {
        let union = Type::Union(vec![Type::string(), Type::integer()]);
        assert_eq!(union.show(), "String | Integer");
    }

    #[test]
    fn test_generic_type_show() {
        let array_int = Type::array_of(Type::integer());
        assert_eq!(array_int.show(), "Array[Integer]");

        let hash_str_int = Type::hash_of(Type::string(), Type::integer());
        assert_eq!(hash_str_int.show(), "Hash[String, Integer]");
    }

    #[test]
    fn test_base_class_name() {
        assert_eq!(Type::string().base_class_name(), Some("String"));
        assert_eq!(Type::array_of(Type::integer()).base_class_name(), Some("Array"));
        assert_eq!(Type::Nil.base_class_name(), None);
        assert_eq!(Type::Bot.base_class_name(), None);
    }

    #[test]
    fn test_type_args() {
        let array_int = Type::array_of(Type::integer());
        let args = array_int.type_args().unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].show(), "Integer");

        // Non-generic types have no type args
        assert!(Type::string().type_args().is_none());
    }
}
