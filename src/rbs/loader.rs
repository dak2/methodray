use crate::env::GlobalEnv;
use crate::rbs::converter::RbsTypeConverter;
use crate::rbs::error::RbsError;
use crate::types::Type;
use magnus::{Error, RArray, RHash, Ruby, TryConvert, Value};

/// Method information loaded from RBS
#[derive(Debug, Clone)]
pub struct RbsMethodInfo {
    pub receiver_class: String,
    pub method_name: String,
    pub return_type: Type,
}

/// Loader that calls RBS API via magnus to load method information
pub struct RbsLoader<'a> {
    ruby: &'a Ruby,
}

impl<'a> RbsLoader<'a> {
    /// Create a new RbsLoader
    /// Assumes RBS gem is already loaded on the Ruby side
    pub fn new(ruby: &'a Ruby) -> Result<Self, RbsError> {
        // Check if RBS module is defined
        let rbs_defined: bool = ruby
            .eval("defined?(RBS) == 'constant'")
            .map_err(|e| RbsError::LoadError(format!("Failed to check RBS: {}", e)))?;

        if !rbs_defined {
            return Err(RbsError::RbsNotInstalled);
        }

        Ok(Self { ruby })
    }

    /// Load all method definitions from RBS
    pub fn load_methods(&self) -> Result<Vec<RbsMethodInfo>, RbsError> {
        // Load method_loader.rb
        let rb_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/rbs/method_loader.rb");
        let load_code = format!("require '{}'", rb_path);
        let _: Value = self
            .ruby
            .eval(&load_code)
            .map_err(|e| {
                RbsError::LoadError(format!("Failed to load method_loader.rb: {}", e))
            })?;

        // Instantiate Rbs::MethodLoader class and call method
        let results: Value = self
            .ruby
            .eval("Rbs::MethodLoader.new.load_methods")
            .map_err(|e| {
                RbsError::LoadError(format!("Failed to call Rbs::MethodLoader#load_methods: {}", e))
            })?;

        self.parse_results(results)
    }

    /// Convert Ruby array results to Vec of RbsMethodInfo structs
    fn parse_results(&self, results: Value) -> Result<Vec<RbsMethodInfo>, RbsError> {
        let mut method_infos = Vec::new();

        // Convert to Ruby array
        let results_array = RArray::try_convert(results)
            .map_err(|e| RbsError::ParseError(format!("Failed to convert to array: {}", e)))?;

        // Process each element
        for entry in results_array.into_iter() {
            let entry: Value = entry;

            // Convert to hash
            let hash = RHash::try_convert(entry).map_err(|e| {
                RbsError::ParseError(format!("Failed to convert entry to hash: {}", e))
            })?;

            // Get each field
            let receiver_class_value = hash
                .get(self.ruby.to_symbol("receiver_class"))
                .ok_or_else(|| RbsError::ParseError("Missing receiver_class".to_string()))?;
            let receiver_class: String = String::try_convert(receiver_class_value).map_err(|e| {
                RbsError::ParseError(format!("Failed to convert receiver_class: {}", e))
            })?;

            let method_name_value = hash
                .get(self.ruby.to_symbol("method_name"))
                .ok_or_else(|| RbsError::ParseError("Missing method_name".to_string()))?;
            let method_name: String = String::try_convert(method_name_value).map_err(|e| {
                RbsError::ParseError(format!("Failed to convert method_name: {}", e))
            })?;

            let return_type_value = hash
                .get(self.ruby.to_symbol("return_type"))
                .ok_or_else(|| RbsError::ParseError("Missing return_type".to_string()))?;
            let return_type_str: String = String::try_convert(return_type_value).map_err(|e| {
                RbsError::ParseError(format!("Failed to convert return_type: {}", e))
            })?;

            // Convert RBS type string to internal Type enum
            let return_type = RbsTypeConverter::parse(&return_type_str);

            method_infos.push(RbsMethodInfo {
                receiver_class,
                method_name,
                return_type,
            });
        }

        Ok(method_infos)
    }
}

/// Helper function to register RBS methods to GlobalEnv
pub fn register_rbs_methods(genv: &mut GlobalEnv, ruby: &Ruby) -> Result<usize, Error> {
    let loader = RbsLoader::new(ruby)?;
    let methods = loader.load_methods()?;
    let count = methods.len();

    for method_info in methods {
        let receiver_type = Type::Instance {
            class_name: method_info.receiver_class,
        };
        genv.register_builtin_method(
            receiver_type,
            &method_info.method_name,
            method_info.return_type,
        );
    }

    eprintln!("Loaded {} methods from RBS", count);
    Ok(count)
}
