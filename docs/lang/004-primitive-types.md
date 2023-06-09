- **Feature Name:** `primitive-types` 
- **Zom Issue:** *Number doesn't correspond* [`zom#010`](https://github.com/zom-lang/zom/issues/10)
- **Status:** `Not implemented yet`

# Primitive types

Primitive types are types that are directly encoded in Zom.

## Signed

Signed numbers work with [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
|  Name  |             Description             |                           Range                          | Default?  |
| ------ | ----------------------------------- | -------------------------------------------------------- | --------- |
| `i8`   | `i8` is a 8 bits signed integer,    | -127 to +127                                             |     No    |
| `i16`  | `i16` is a 16 bits signed integer   | −32_767 to +32_767                                       |     No    |
| `i32`  | `i32` is a 32 bits signed integer   | −2_147_483_647 to +2_147_483_647                         |    Yes    |
| `i64`  | `i64` is a 64 bits signed integer   | −9_223_372_036_854_775_807 to +9_223_372_036_854_775_807 |   Maybe   |
| `i128` | `i128` is a 128 bits signed integer | -170_141_183_460_469_231_731_687_303_715_884_105_728 <br /> to +170_141_183_460_469_231_731_687_303_715_884_105_727 |   Maybe   |
> By default numbers are signed but, you can use the "signed" keyword to explicitly say it's signed (that's why in the table `signed` is in brackets).
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.
You can type a number like that `123456` or like that if the number is big or it's complex to read it `123_456`.

## Unsigned

|  Name  |           Description                |              Range              | Default? |
| ------ | ------------------------------------ | ------------------------------- | -------- |
| `u8`   | `u8` is a 8 bits unsigned integer    | 0 to 255                        |    No    |
| `u16`  | `u16` is a 16 bits unsigned integer  | 0 to 65_535                     |    No    |
| `u32`  | `u32` is a 32 bits unsigned integer  | 0 to 4_294_967_295              |    No    |
| `u64`  | `u64` is a 64 bits unsigned integer  | 0 to 18_446_744_073_709_551_615 |    No    |
| `i128` | `i128` is a 128 bits signed integer  | 0 to 340_282_366_920_938_463_463_374_607_431_768_211_455 |   No   |
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.

## Platform Spcific integer

The pointer-sized integer type.

The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.

|  Name   |       Description        |
| ------- | ------------------------ |
| `usize` | `usize` unsigned integer |
| `isize` | `isize` signed integer   |

## Floating Point Number

Floating numbers, specified in the [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) 
|  Name |                      Description                     | Default? |                             Range                           |
| ----- | ---------------------------------------------------- | -------- |  ---------------------------------------------------------- |
| `f32` | `f32` is a single precision floating number (32bits) |    Yes   | [`≈ 1.18 × 10^-38` to `≈ 3.4028235 × 10^38`][f32-wikipedia] |
| `f64` | `f64` is a double precision floating number (64bits) |    No    | [`≈ -7.2 × 10^75` to `≈ 7.2 × 10^75`][f64-wikipedia]        |
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.

[f32-wikipedia]: https://en.wikipedia.org/wiki/Single-precision_floating-point_format
[f64-wikipedia]: https://en.wikipedia.org/wiki/Double-precision_floating-point_format

You can type a float number like that `123456,789012` or like that if the number is too big or it's complex to read it, `123_456,789_012`

Others primitive types,
|  Name  |                           Description                          |
| ------ | -------------------------------------------------------------- |
| `bool` | `bool` is a boolean value; either set to true (1) or false (0) |
| `char` | `char` is a Unicode scalar value, that is 4 bytes each         |
| `str`  | `str` is a UTF-8-encoded,                                      | 

Boolean is either `true` or `false`.

A char is initialized with an apostrophe and the char in between, like that : `'A'`.

A string is initialized with a quotation mark and the string in between, like that : `"Hello, world!"`.
