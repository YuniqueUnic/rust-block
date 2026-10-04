use super::*;

#[test]
fn test_int_block() {
    unsafe {
        assert!(invoke_int_block(get_int_block()) == 7);
        assert!(invoke_int_block(get_int_block_with(13)) == 13);
    }
}

#[test]
fn test_add_block() {
    unsafe {
        assert!(invoke_add_block(get_add_block(), 5) == 12);
        assert!(invoke_add_block(get_add_block_with(3), 5) == 8);
    }
}
