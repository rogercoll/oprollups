pub struct Transaction {
    nonce: u8,
    gas_price: u8,
    gas: u8,
    to: u32, //4 Byte address
    from: u32,
    pub value: u32,
    pub signature: u8,
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