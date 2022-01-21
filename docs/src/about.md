Welcome to the OurAirports JSON API documentation. Here you can find the URL for each data, the available queries as
well as the fields and its data type.

If you're looking for documentation of the `ourairports_api` crate, you should go to the rust documentation in the home
page instead.

For more information about what each field contains, you should go to
the [OurAirports data dictionary](https://ourairports.com/help/data-dictionary.html).

## Legend on data types

`T` is a generic type parameter that can be substituted as one of the non-generic data types.

|Type|Description|Equivalent Rust type|
|----|-----------|--------------------|
|`Int8`|Signed 8-bit integer|`i8`
|`Int16`|Signed 16-bit integer|`i16`
|`Int32`|Signed 32-bit integer|`i32`
|`Int64`|Signed 64-bit integer|`i64`
|`Int128`|Unsigned 8-bit integer|`i128`
|`Uint8`|Unsigned 8-bit integer|`u8`
|`Uint16`|Unsigned 16-bit integer|`u16`
|`Uint32`|Unsigned 32-bit integer|`u32`
|`Uint64`|Unsigned 64-bit integer|`u64`
|`Uint128`|Unsigned 128-bit integer|`u128`
|`Float32`|32-bit floating point|`f32`
|`Float64`|64-bit floating point|`f64`
|`String`|String, encoded in UTF-8|`String`
|`Bool`|Boolean values (`true` or `false`)|`bool`
|`[T]`|An array of type `T`|`Vec<T>`
|`T?`|A nullable value of type `T`|`Option<T>`

