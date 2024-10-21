use std::fmt::Display;
use windows::core::HSTRING;
use crate::custom_windows::utils::trim_wide_null;

#[derive(Debug)]
pub struct WideString {
    value: Vec<u16>,
}

impl WideString {
    pub fn new(length: usize) -> Self {
        let mut initial_vec : Vec<u16> = Vec::new();
        if length > 0 {
            initial_vec.resize(length, 0);
        }

        Self {value: initial_vec}
    }
    
    pub fn trim_null(&mut self) -> &mut Self {
        self.value = Vec::from(trim_wide_null(&self.value));
        self
    }

    fn slice_null(&self) -> &[u16] {
        let index = self.value.iter().position(|&c| c == 0);

        if let Some(i) = index {
            &self.value[0..i].iter().as_slice()
        } else {
            &self.value
        }
    }

    pub fn to_h_string(&self) -> windows::core::Result<HSTRING> {
        HSTRING::from_wide(self.slice_null())
    }

    pub fn as_mut_u16(&mut self) -> &mut [u16] {
        self.value.as_mut_slice()
    }
}

impl From<&[u16]> for WideString {
    fn from(value: &[u16]) -> Self {
        Self { value: Vec::from(value) }
    }
}

impl Display for WideString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_h_string().unwrap())
    }
}