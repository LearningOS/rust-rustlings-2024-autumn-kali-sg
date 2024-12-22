// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    // todo!("The rest of the code goes here")
    ret.b = Some("hello".to_owned());
    ret
    /* unsafe { Box::from_raw(ptr) }：这是不安全的，因为 Box::from_raw 将原始指针转换回 Box<T>。
     * 这要求 ptr 必须确实指向一个之前由 Box::into_raw 创建的 Box<Foo>，否则会导致未定义行为。
     * 这个操作不会执行任何内存分配或释放；
     * 它只是简单地将原始指针重新解释为 Box<Foo>，因此需要确保 ptr 指向的是有效的、由 Box 分配的对象。 */
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
