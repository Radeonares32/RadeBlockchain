use serde_json::Value;
use serde::{Deserializer,Serializer};

pub struct Block {
    recipient:String,
    amount:u8,
    gas:u8,
    additional_data:Value,
    nonce:u8,
    signature:Option<Value>
}

impl Block {
    pub fn new(recipient:String,amount:u8,gas:u8,additional_data:Value,nonce:u8,signature:Value) -> Self {
        Self {
            recipient,
            amount,
            gas,
            additional_data,
            nonce,
            signature:None
        }
    }
    pub fn default() -> Self {
        Self {
            recipient:String::from("")
            amount:String::from("0"),
            gas:String::from("0"),
            additional_data:Value,
            nonce:0,
            signature:None
            
        }
    }
    pub fn serialize(tx:Block) -> Vec<u8> {
        let mut tx_hex = "";
        tx_hex += Self::pad_start(tx.recipient, 64, b'0');
        tx_hex = Self::pad_start(tx.amount, 16, b'0');
        
        
        
    }
    fn pad_start(data: &[u8], len: usize, pad_char: u8) -> Vec<u8> {
        let pad_len = len.saturating_sub(data.len());
        let mut padded_data = vec![pad_char; pad_len];
        padded_data.extend_from_slice(data);
        padded_data
    }
}