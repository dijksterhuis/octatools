//! Common functions/ traits / implementations across Octatrack data structures.

// LEARN: Must be imported to use these methods.
// TODO: from_value and value method generic type arguments

/// Trait to convert between Enum option instances and their corresponding value.

pub trait OptionEnumValueConvert {

    /// One of the enum types within the `octatrack::options` module.
    type T;

    /// Input type for `from_value` and return type for `value` method.
    type V;
    
    /// Get an Enum instance from a numeric value.
    fn from_value(v: Self::V) -> Result<Self::T, ()>;

    /// Get a numeric value for an Enum instance.
    fn value(&self) -> Result<Self::V, ()>;

}
