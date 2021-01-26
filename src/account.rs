pub struct Account {
    pub id: u32,
    pub value: u32,
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
        let idu8 = transform_u32_to_array_of_u8(self.id);
        let valueu8 = transform_u32_to_array_of_u8(self.value);
        let array: [u8; 8] = {
            let mut x = [0; 8];
            x[0] = idu8[0]; 
            x[1] = idu8[1];
            x[2] = idu8[2]; 
            x[3] = idu8[3];
            x[4] = valueu8[0]; 
            x[5] = valueu8[1];
            x[6] = valueu8[2]; 
            x[7] = valueu8[3];
            x
        };
        blake3::hash(&array)
    }
}

