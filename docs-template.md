# Documentation Template

This file provides templates for adding documentation comments to Rust code.

## Module Documentation

Add this at the top of each module file:

```rust
//! Module for [brief description].
//!
//! This module provides [detailed description of what the module does].
//!
//! # Examples
//!
//! ```
//! // Example code here
//! ```

// Module code...
```

## Struct Documentation

```rust
/// Represents [what the struct represents].
///
/// # Fields
///
/// * `field1` - Description of field1
/// * `field2` - Description of field2
///
/// # Examples
///
/// ```
/// let item = StructName {
///     field1: value1,
///     field2: value2,
/// };
/// ```
pub struct StructName {
    pub field1: Type1,
    pub field2: Type2,
}
```

## Function Documentation

```rust
/// Brief description of what the function does.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// let result = function_name(arg1, arg2);
/// ```
///
/// # Errors
///
/// Returns `Err` if [condition]
pub fn function_name(arg1: Type1, arg2: Type2) -> Result<ReturnType, Error> {
    // Implementation
}
```

## Enum Documentation

```rust
/// Represents [what the enum represents].
///
/// # Variants
///
/// * `Variant1` - Description
/// * `Variant2` - Description
///
/// # Examples
///
/// ```
/// let value = EnumName::Variant1;
/// ```
pub enum EnumName {
    /// First variant
    Variant1,
    /// Second variant
    Variant2,
}
```

## Documentation Guidelines

1. **Use `///` for item documentation** (structs, functions, enums)
2. **Use `//!` for module documentation** (at the top of files)
3. **Include examples when possible**
4. **Document public APIs thoroughly**
5. **Use markdown formatting**
6. **Cross-reference with `[Type]` or `[function_name]`**

## Useful Sections

- `# Examples` - Code examples
- `# Arguments` - Function parameters
- `# Returns` - Return values
- `# Errors` - Error conditions
- `# Panics` - When the code might panic
- `# Safety` - For unsafe code
- `# See also` - Related items

## Testing Documentation

Run doc tests:
```bash
cargo test --doc
```

Generate documentation:
```bash
cargo doc --open
```

Check for broken links:
```bash
cargo rustdoc -- -D warnings
```
