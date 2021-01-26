
#[derive(Hash)]
pub struct Transaction {
    nonce: u8,
    gas_price: u8,
    gas: u8,
    to: u32, //4 Byte address
    from: u32,
    pub value: u32,
    pub signature: u8,
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

impl Transaction {
    pub fn sign(&mut self) {
        self.signature = 3
    }
}

pub fn new(_to: u32, _from: u32, _value: u32) -> Transaction {
    Transaction {
        nonce: 0,
        gas_price: 0,
        gas: 0,
        to: _to,
        from: _from,
        value: _value,
        signature: 0
    }
}