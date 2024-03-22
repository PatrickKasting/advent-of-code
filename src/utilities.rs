pub fn as_isize(value: usize) -> isize {
    value
        .try_into()
        .expect("value should be less than 'isize::MAX'")
}
