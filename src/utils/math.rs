use hex;

pub struct Math {}

impl Math {
    pub fn log16(data:f64) -> Result<f64,&'static str> {
        if data.is_sign_positive() {
            Ok(data.ln() / 16.0_f64.ln())
        }
        else {
            Err("x not positive")
        }
    }
    pub fn is_number(data:&str) -> bool {
        data.chars().all(|c| c.is_digit(10))
    }
    pub fn is_hex(data:&str) -> bool {
        data.starts_with("0x") && data[2..].chars().all(|c| c.is_digit(10))
    }
    pub fn num_to_buffer(data:&str) -> Vec<u8> {
        let hex_value = data.as_bytes();
        let hex_length = hex_value.len() + (hex_value.len() % 2 != 0) as usize;
        let padded_hex = Self::pad_start(hex_value, hex_length, b'0');
        hex::decode(padded_hex).unwrap()
    }
    fn pad_start(data: &[u8], len: usize, pad_char: u8) -> Vec<u8> {
        let pad_len = len.saturating_sub(data.len());
        let mut padded_data = vec![pad_char; pad_len];
        padded_data.extend_from_slice(data);
        padded_data
    }
}
