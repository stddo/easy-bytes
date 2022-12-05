/// Returns struct field's size in bytes.
///
/// ## Example
/// ```
/// use easy_bytes::field_size;
///
/// struct Foo(i32);
///
/// assert_eq!(field_size!(Foo[0]), 4)
/// ```
#[macro_export]
macro_rules! field_size {
    ($a:ty[$b:ident]) => {{
        $crate::return_type_size(&|a: $a| a.$b)
    }};

    ($a:ty[$b:tt]) => {{
        $crate::return_type_size(&|a: $a| a.$b)
    }};
}

pub const fn return_type_size<T, U>(_: &impl FnOnce(T) -> U) -> usize {
    std::mem::size_of::<U>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        foo: u32,
        bar: Vec<u16>
    }

    struct TestTupleStruct(i16);

    #[test]
    fn primitive_type_size() {
        let size = field_size!(TestStruct[foo]);
        assert_eq!(size, 4);
    }

    #[test]
    fn complex_type_size() {
        let size = field_size!(TestStruct[bar]);
        assert_eq!(size, 24);
    }

    #[test]
    fn tuple_struct_field_size() {
        let size = field_size!(TestTupleStruct[0]);
        assert_eq!(size, 2);
    }
}
