pub struct Fnv1Hasher {
    state: u64,
}

impl std::hash::Hasher for Fnv1Hasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            (self.state, _) = self.state.overflowing_mul(1099511628211);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub struct BuildFnv1Hasher;

impl std::hash::BuildHasher for BuildFnv1Hasher {
    type Hasher = Fnv1Hasher;
    fn build_hasher(&self) -> Self::Hasher {
        Fnv1Hasher {
            state: 14695981039346656037,
        }
    }
}
