use core::hash::{BuildHasher, Hasher};
use core::marker::PhantomData;

macro_rules! gbuild {
    ($algo:ty: $init:expr) => {
        impl BuildHasher for $algo {
            type Hasher = G64<$algo>;
            fn build_hasher(&self) -> G64<$algo> {
                G64($init, PhantomData)
            }
        }
    };
    () => {fn finish(&self) -> u64 {self.0}}
}

struct G64<T>(u64, PhantomData<T>);

struct FNV;

impl Hasher for G64<FNV> {
    gbuild![];

    fn write(&mut self, stream: &[u8]) {
        for byte in stream {
            self.0 ^= *byte as u64;
            self.0 *= 0x100000001B3;
        }
    }
}

gbuild!(FNV: 0xcbf29ce484222325);

struct Jenkins;

impl Hasher for G64<Jenkins> {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, vals: &[u8]) {
        for val in vals {
            self.0 += *val as u64;
            self.0 += self.0 << 10;
            self.0 ^= self.0 >> 6;
        }
        self.0 += self.0 << 3;
        self.0 ^= self.0 >> 11;
        self.0 += self.0 << 15;
    }
}

gbuild!(Jenkins: 0);
