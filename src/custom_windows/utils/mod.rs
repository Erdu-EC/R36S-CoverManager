pub mod pcwstr;
pub mod wide_string;

pub fn trim_wide_null(value: &[u16]) -> &[u16] {
    let index = value.iter().position(|&c| c == 0);

    if let Some(i) = index {
        &value[0..i].iter().as_slice()
    } else {
        &value
    }
}
