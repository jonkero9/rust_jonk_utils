#[derive(Debug)]
pub struct Jrand {
    pub seed: u64,
}

impl Jrand {
    pub fn new() -> Jrand {
        return Jrand { seed: 0 };
    }

    pub fn rnd(&mut self) -> u32 {
        self.seed = self.seed.wrapping_add(0xe120fc15);
        let mut tmp: u64 = self.seed.wrapping_mul(0x4a39b70d8);
        let m1: u32 = ((tmp >> 32) ^ tmp) as u32;
        tmp = (m1 as u64).wrapping_mul(0x12fad5c9);
        let m2: u32 = ((tmp >> 32) ^ tmp) as u32;
        return m2;
    }

    pub fn rnd_range(&mut self, min: u32, max: u32) -> u32 {
        return (self.rnd() % (max - min)) + min;
    }

    pub fn rnd_range_float(&mut self, min: f32, max: f32) -> f32 {
        return (self.rnd() as f32 / u32::MAX as f32) * (max - min) + min;
    }
}

pub fn cantor_hash(a: i32, b: i32) -> u64 {
    let x: u64 = match a >= 0 {
        true => 2 * a as u64,
        false => (-2 * a - 1) as u64,
    };
    let y: u64 = match b >= 0 {
        true => 2 * b as u64,
        false => (-2 * b - 1) as u64,
    };
    let z: i64 = match x >= y {
        true => ((x * x + x + y) / 2) as i64,
        false => ((x + y * y) / 2) as i64,
    };
    return match (a < 0 && b < 0) || (a >= 0 && b >= 0) {
        true => z as u64,
        false => (-z - 1) as u64,
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn rnd_test_seed_determinism_test() {
        let mut o = Jrand::new();
        let mut expected: Vec<u32> = Vec::new();
        let mut results: Vec<u32> = Vec::new();
        o.seed = 1;
        for _i in 1..20 {
            expected.push(o.rnd());
        }
        o.seed = 1;
        for _i in 1..20 {
            results.push(o.rnd());
        }
        assert_eq!(expected, results);
    }

    #[test]
    fn rnd_range_test() {
        let mut o = Jrand::new();
        for _i in 0..2000 {
            let result = o.rnd_range(4, 50);
            assert!(result >= 4 && result < 50);
        }
    }

    #[test]
    fn rnd_range_float_test() {
        let mut o = Jrand::new();
        for _i in 0..2000 {
            let result = o.rnd_range_float(4., 50.);
            assert!(result >= 4. && result < 50.);
        }
    }

    #[test]
    fn cantor_hash_test_collision_test() {
        let mut result_set: HashSet<u64> = HashSet::new();
        let iters = 20;
        for i in -iters..iters {
            for j in -iters..iters {
                assert!(result_set.insert(cantor_hash(i, j)));
            }
        }
    }
}
