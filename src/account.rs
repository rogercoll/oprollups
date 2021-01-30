//use bn::{Group, Fr, G1, G2, pairing};
use bls_signatures_rs::bn256;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

//Balance in weis
pub struct Account {
    pub addr: [u8; 64],
    pub balance: u32,
    id: u32,
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

impl Account {
    pub fn hash(&self) -> blake3::Hash {
        //let addr_u8 = transform_u32_to_array_of_u8(self.addr);
        let balance_u8 = transform_u32_to_array_of_u8(self.balance);
        let array: [u8; 68] = {
            let mut x = [0; 68];
            for i in 0..self.addr.len() {
                x[i] = self.addr[i];
            }
            x[64] = balance_u8[0]; 
            x[65] = balance_u8[1];
            x[66] = balance_u8[2]; 
            x[67] = balance_u8[3];
            x
        };
        blake3::hash(&array)
    }
    pub fn hash_addr(&self) -> String {
        let mut hasher = Sha3::sha3_256();
        hasher.input(&self.addr);
        hasher.result_str()
    }
    pub fn set_id(&mut self, _id: u32) {
        self.id = _id;
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

pub fn hex_addr(addr: &[u8; 32]) -> String {
    hex::encode(addr)
}


fn vector_as_u8_64_array(vector: Vec<u8>) -> [u8;64] {
    let mut arr = [0u8;64];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}

pub fn new(_addr: u32, _balance: u32) -> Account {
    let sk: [u8; 32]  = rand::random();
    let pk1 = bn256::PrivateKey::new(&sk).unwrap();
    println!("Private key: {:?}\n", hex::encode(pk1.to_bytes().unwrap()));
    let pubkey = pk1.derive_public_key().unwrap();

    Account {
        addr: vector_as_u8_64_array(pubkey.to_compressed().unwrap()),
        balance: _balance,
        id: 0,
    }
}

