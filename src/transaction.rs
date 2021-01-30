use bls_signatures_rs::bn256::Bn256;
use bls_signatures_rs::MultiSignature;


pub struct Transaction {
    gas: u8,
    gas_price: u8,
    pub to: u32, //4 Byte address Is an index identifing the corresponding leaf in the merkle tree
    pub from: u32, //we can avoit this field as can be recovered from the signature
    pub value: u32,
    pub signature: [u8;32],
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn vector_as_u8_32_array(vector: Vec<u8>) -> [u8;32] {
    let mut arr = [0u8;32];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}

impl Transaction {
    pub fn sign(&mut self, sk: &str) {
        let secret_key_1 = hex::decode(sk).unwrap();
        let sig_1 = Bn256.sign(&secret_key_1, &self.to_array_u8()).unwrap();
        self.signature = vector_as_u8_32_array(sig_1);
    }
    fn to_array_u8(&self) -> [u8; 10] {
        let mut array: [u8;10]  = [0u8; 10];
        array[0] = self.gas;
        array[1] = self.gas_price;
        let to_array = transform_u32_to_array_of_u8(self.to);
        let value_array = transform_u32_to_array_of_u8(self.to);
        array[2] = to_array[0];
        array[3] = to_array[1];
        array[4] = to_array[2];
        array[5] = to_array[3];
        array[6] = value_array[0];
        array[7] = value_array[1];
        array[8] = value_array[2];
        array[9] = value_array[3];
        array
    }
}

pub fn new(_to: u32, _from: u32, _value: u32) -> Transaction {
    Transaction {
        gas_price: 0,
        gas: 0,
        to: _to,
        from: _from,
        value: _value,
        signature: [0u8; 32]
    }
}