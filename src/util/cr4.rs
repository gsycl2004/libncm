

pub struct CR4 {
    sbox:[i32;256]
}


impl CR4 {
    pub fn new()-> CR4 {
        CR4 { sbox:[0;256] }
    }
    
    pub fn ksa(&mut self,key:&[u8]){
        let length = key.len();
        for i in 0..256 {
            self.sbox[i] = i as i32;
        }
        let mut j = 0;
        for i in 0.. 256 {
            j = (j + self.sbox[i] + key[i % length] as i32) & 0xff;
            let f = &mut self.sbox;
            f.swap(i, j as usize);
        }
    }

    pub fn prga(&self,data:&mut [u8],length:usize){
        let mut i;
        let mut j;
        for k in 0..length{
            i = (k+1) & 0xff;
            j = (self.sbox[i] + i as i32) & 0xff;
            data[k] ^= self.sbox[((self.sbox[i]+self.sbox[j as usize]) & 0xff) as usize] as u8
        }
    }
}