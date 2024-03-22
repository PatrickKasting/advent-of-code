pub fn char_at<S: AsRef<str>>(str: S, index: usize) -> char {
    str.as_ref()
        .chars()
        .nth(index)
        .expect("char at given index should exist")
}

pub fn as_isize(value: usize) -> isize {
    value
        .try_into()
        .expect("value should be less than 'isize::MAX'")
}
