use std::collections::HashMap;

use super::{
    constants::{IppDelimiterTag, IppOperationId},
    encode::encode,
    structs::{Attribute},
};

pub struct Operation {
    pub id: IppOperationId,
    pub uri: String,
    pub data: Vec<u8>,
    pub attribute_group: HashMap<IppDelimiterTag, Vec<Attribute>>,
}

impl Operation {
    pub fn as_ipp(&self) -> Vec<u8> {
        return encode(self);
    }

    pub fn send(&self) -> bool {
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("Printers (Rust library)")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let res = client
            .post(self.uri.replace("ipp", "http").as_str())
            .header("Content-Type", "application/ipp")
            .body(self.as_ipp())
            .send();

        if res.is_err() {
            println!("E => {:?}", res.unwrap_err());

            return false;
        } else {
            let response = res.unwrap().text().unwrap();
            println!("C => {:?}", response.as_bytes());

            return true;
        }
    }

}
