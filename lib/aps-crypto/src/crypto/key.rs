//!
//! Crypto Key structure.
//!
//! This is a simple wrapper to [u8; 32] for representing an AES128 Key with creation length checks.
//!

#[derive(Copy, Clone, Eq, Debug)]
pub struct Key([u8; 32]);

impl PartialEq<Self> for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Into<[u8; 32]> for Key{
    fn into(self) -> [u8; 32] {
        return self.0
    }
}

impl Into<Key> for [u8; 32] {
    fn into(self) -> Key {
        return Key(self);
    }
}

impl TryInto<Key> for &[u8] {
    type Error = ();

    fn try_into(self) -> Result<Key, Self::Error> {
        if self.len() < 32 {
            return Err(())
        }

        let arr: Result<[u8; 32], _> = self[0..32].try_into();
        if let Ok(a) = arr {
            return Ok(Key(a))
        }
        return Err(())
    }
}


impl Key {
    /// instantiates a key from an array of length 32
    pub fn from_arr(x: [u8; 32]) -> Self {
        x.into()
    }

    /// instantiates a key from a slice.
    ///
    /// if the slice is too short it will panic
    /// if the slice is too long the key will be instantiated from the first 32 bytes
    pub fn from_slice(x: &[u8]) -> Result<Self, ()> {
        x.try_into()
    }


    pub fn to_array(self) -> [u8; 32] {
        self.into()
    }

    pub fn to_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn empty() -> Self {
        Self::from_arr([0u8; 32])
    }
}





#[cfg(test)]
mod key_test {
    extern crate std;
    use crate::crypto::key::Key;

    #[test]
    fn equality(){
        let rka: Vec<u8> = (0..32).map(|i| i as u8).collect();
        let rkb: Vec<u8> = (0..32).map(|i| i as u8).collect();
        let rkc: Vec<u8> = (32..64).map(|i| i as u8).collect();

        let raw_key_vec_a: &[u8] = &rka;
        let raw_key_vec_b: &[u8] = &rkb;
        let raw_key_vec_c: &[u8] = &rkc;

        let ka: Key = raw_key_vec_a.try_into().expect("Key should be valid");
        let kb: Key = raw_key_vec_b.try_into().expect("Key should be valid");
        let kc: Key = raw_key_vec_c.try_into().expect("Key should be valid");

        assert_eq!(ka, kb);
        assert_ne!(ka, kc);
        assert_ne!(kb, kc);
    }

    #[test]
    fn from_array(){
        let k = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1 ,2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let key = Key::from_arr(k);
        assert_eq!(k, key.0)
    }

    #[test]
    fn from_slice(){
        let k = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1 ,2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let k_long = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1 ,2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 9, 10, 11];
        let k_short = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1 ,2, 3, 4, 5, 6, 7];

        let key = Key::from_slice(&k);
        assert!(key.is_ok());
        assert_eq!(key.as_ref().unwrap().0, k);

        let key = Key::from_slice(&k_long);
        assert!(key.is_ok());
        assert_eq!(key.as_ref().unwrap().0, k);

        let key = Key::from_slice(&k_short);
        assert!(key.is_err());
    }

}