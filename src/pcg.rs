use rand::{Rng, SeedableRng};

/// A [PCG](http://www.pcg-random.org) random number generator.
///
/// The PCG algorithm is not suitable for cryptographic purposes but provides an
/// excellent balance of speed and unpredictability.
pub struct PcgRng {
    state: u64,
    inc: u64,
}

impl PcgRng {
    /// Returns a new `PcgRng` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created
    /// by this function will yield the same stream of random numbers. It is
    /// highly recommended that this is created through `SeedableRng` instead of
    /// this function.
    pub fn new_unseeded() -> PcgRng {
        PcgRng {
            state: 0x853c49e6748fea9b,
            inc: 0xda3e39cb94b95bdb,
        }
    }
}

impl Rng for PcgRng {
    #[allow(unsigned_negation)]
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old * 6364136223846793005 + self.inc;
        let xor = (((old >> 18) ^ old) >> 27) as u32;
        let rot = old >> 59 as u32;
        let out = (xor >> rot) | (xor << ((-rot) & 31));
        out
    }
}

impl SeedableRng<[u64; 2]> for PcgRng {
    /// Reseed a `PcgRng`. This will panic if `seed` is entirely 0.
    fn reseed(&mut self, seed: [u64; 2]) {
        assert!(!seed.iter().all(|&x| x == 0),
                "PcgRng.reseed called with an all zero seed.");
        self.state = 0;
        self.inc = (seed[1] << 1) | 1;
        self.next_u32();
        self.state += seed[0];
        self.next_u32();
    }

    /// Create a new `PcgRng`. This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u64; 2]) -> PcgRng {
        assert!(!seed.iter().all(|&x| x == 0),
                "PcgRng::from_seed called with an all zero seed.");

        let mut rng = PcgRng {
            state: 0,
            inc: (seed[1] << 1) | 1,
        };
        rng.next_u32();
        rng.state += seed[0];
        rng.next_u32();
        rng
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::{Rng, SeedableRng};

    #[test]
    fn output() {
        let mut rng = PcgRng::from_seed([42, 54]);

        let v: Vec<u32> = rng.gen_iter().take(6).collect();

        // test vectors from pcg32-global-demo
        assert_eq!(v, vec![0xa15c02b7, 0x7b47f409, 0xba1d3330,
                           0x83d2f293, 0xbfa4784b, 0xcbed606e]);
    }
}
