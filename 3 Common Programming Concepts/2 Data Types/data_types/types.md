# Scalar

Represents a single value.

4 primary types: integers, floating-point, booleans, characters

## Integers

A number without a fractional component

### Int Table

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

## Floating-point

two types: f32 and f64

Rust defaults to f64

## Boolean

true and false

## Character

char is the most primitive alphabetic type

Unicode scalar value, not ASCII

# Compound Types

These can group multiple values into one type.

2 primative types: tuples and arrays

## Tuple

Fixed length

Stored on heap

Each item in a tuple can be of a different type.

Can optionally declasre the type in parentheses

Use pattern matching to destructure a tuple

Use period followed by an index (x.2) to access the value we want

A tuple without any values is called a unit.  This value and type are both written () and represent an empty value or empty return type

## Array

Fixed length

Stored on the stack

Access using indexing