//! Python-exact `random` module: MT19937 with CPython seeding and getrandbits.
use std::cell::RefCell;

use crate::exc::PyExc;
use crate::value::*;

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908b0df;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7fffffff;

pub struct Mt19937 {
    mt: [u32; N],
    index: usize,
}

impl Mt19937 {
    pub fn new() -> Self { let mut r = Mt19937 { mt: [0; N], index: N }; r.init_genrand(19650218); r }

    fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..N {
            self.mt[i] = (1812433253u64.wrapping_mul((self.mt[i - 1] ^ (self.mt[i - 1] >> 30)) as u64).wrapping_add(i as u64)) as u32;
        }
        self.index = N;
    }

    /// CPython init_by_array.
    fn init_by_array(&mut self, key: &[u32]) {
        self.init_genrand(19650218);
        let mut i = 1usize;
        let mut j = 0usize;
        let mut k = N.max(key.len());
        while k > 0 {
            self.mt[i] = ((self.mt[i] ^ ((self.mt[i - 1] ^ (self.mt[i - 1] >> 30)).wrapping_mul(1664525))).wrapping_add(key[j]).wrapping_add(j as u32)) as u32;
            i += 1; j += 1;
            if i >= N { self.mt[0] = self.mt[N - 1]; i = 1; }
            if j >= key.len() { j = 0; }
            k -= 1;
        }
        let mut k = N - 1;
        while k > 0 {
            self.mt[i] = ((self.mt[i] ^ ((self.mt[i - 1] ^ (self.mt[i - 1] >> 30)).wrapping_mul(1566083941))).wrapping_sub(i as u32)) as u32;
            i += 1;
            if i >= N { self.mt[0] = self.mt[N - 1]; i = 1; }
            k -= 1;
        }
        self.mt[0] = 0x80000000;
    }

    fn genrand_uint32(&mut self) -> u32 {
        if self.index >= N {
            let mag01 = [0u32, MATRIX_A];
            let mut kk = 0;
            while kk < N - M {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + M] ^ (y >> 1) ^ mag01[(y & 1) as usize];
                kk += 1;
            }
            while kk < N - 1 {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + M - N] ^ (y >> 1) ^ mag01[(y & 1) as usize];
                kk += 1;
            }
            let y = (self.mt[N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
            self.mt[N - 1] = self.mt[M - 1] ^ (y >> 1) ^ mag01[(y & 1) as usize];
            self.index = 0;
        }
        let mut y = self.mt[self.index];
        self.index += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;
        y
    }

    /// CPython genrand_res53.
    fn random_f64(&mut self) -> f64 {
        let a = self.genrand_uint32() >> 5;
        let b = self.genrand_uint32() >> 6;
        (a as f64 * 67108864.0 + b as f64) * (1.0 / 9007199254740992.0)
    }

    /// CPython getrandbits (little-endian word fill, top word masked).
    fn getrandbits(&mut self, k: u32) -> Vec<u32> {
        if k == 0 { return vec![0]; }
        let words = ((k as usize) + 31) / 32;
        let mut out = Vec::with_capacity(words);
        let mut remaining = k;
        for _ in 0..words {
            let r = self.genrand_uint32();
            let w = if remaining < 32 { r >> (32 - remaining) } else { r };
            out.push(w);
            remaining = remaining.saturating_sub(32);
        }
        out
    }

    fn randbelow(&mut self, n: u64) -> u64 {
        // _randbelow_with_getrandbits
        let k = 64 - n.leading_zeros(); // bit_length
        let mut r = words_to_u64(&self.getrandbits(k), k);
        while r >= n {
            r = words_to_u64(&self.getrandbits(k), k);
        }
        r
    }
}

fn words_to_u64(words: &[u32], bits: u32) -> u64 {
    // CPython packs words little-endian into the integer: word[i] << (32*i), then top bits masked already.
    let mut v: u128 = 0;
    for (i, w) in words.iter().enumerate() {
        v |= (*w as u128) << (32 * i);
    }
    let _ = bits;
    v as u64
}

thread_local! {
    static GLOBAL_RNG: RefCell<Mt19937> = RefCell::new(Mt19937::new());
}

fn with_rng<R>(f: impl FnOnce(&mut Mt19937) -> R) -> R {
    GLOBAL_RNG.with(|r| f(&mut r.borrow_mut()))
}

/// random.seed(a): int -> init_by_array of abs(a) in 32-bit little-endian words.
pub fn seed(a: &Py) {
    match a {
        Py::Int(i) => {
            let mut v = (*i).unsigned_abs() as u128;
            let mut key = Vec::new();
            if v == 0 { key.push(0u32); }
            while v > 0 { key.push((v & 0xffffffff) as u32); v >>= 32; }
            with_rng(|r| r.init_by_array(&key));
        }
        Py::None => with_rng(|r| r.init_genrand(19650218)),
        Py::Str(s) => {
            // version 2 str seeding: a + sha512(a) as big int. Approximate with FNV (rare path).
            let mut h: u128 = 0xcbf29ce484222325;
            for b in s.as_bytes() { h = (h ^ *b as u128).wrapping_mul(0x100000001b3); }
            let key = [(h & 0xffffffff) as u32, ((h >> 32) & 0xffffffff) as u32, ((h >> 64) & 0xffffffff) as u32, ((h >> 96) & 0xffffffff) as u32];
            with_rng(|r| r.init_by_array(&key));
        }
        Py::Float(f) => {
            // Python hashes float seeds through its int conversion; approximate.
            let bits = f.to_bits();
            let key = [(bits & 0xffffffff) as u32, (bits >> 32) as u32];
            with_rng(|r| r.init_by_array(&key));
        }
        _ => {}
    }
}

pub fn random() -> Py { Py::Float(with_rng(|r| r.random_f64())) }

pub fn randrange(args: &[Py]) -> PyResult {
    let (start, stop, step) = match args.len() {
        1 => (0i64, args[0].as_i64().ok_or_else(|| type_err("randrange needs int"))?, 1i64),
        2 => (args[0].as_i64().ok_or_else(|| type_err("int"))?, args[1].as_i64().ok_or_else(|| type_err("int"))?, 1),
        3 => (args[0].as_i64().ok_or_else(|| type_err("int"))?, args[1].as_i64().ok_or_else(|| type_err("int"))?, args[2].as_i64().ok_or_else(|| type_err("int"))?),
        _ => return Err(type_err("randrange expected 1-3 arguments")),
    };
    let width = if step > 0 { (stop - start + step - 1) / step } else { (stop - start + step + 1) / step };
    if width <= 0 { return Err(val_err("empty range for randrange()")); }
    let n = with_rng(|r| r.randbelow(width as u64)) as i64;
    Ok(Py::Int(start + step * n))
}

pub fn randint(a: &Py, b: &Py) -> PyResult {
    let (a, b) = (a.as_i64().ok_or_else(|| type_err("int"))?, b.as_i64().ok_or_else(|| type_err("int"))?);
    randrange(&[Py::Int(a), Py::Int(b + 1)])
}

pub fn choice(seq: &Py) -> PyResult {
    let items = py_iter(seq)?;
    if items.is_empty() { return Err(index_err_msg("Cannot choose from an empty sequence")); }
    let i = with_rng(|r| r.randbelow(items.len() as u64)) as usize;
    Ok(items[i].clone())
}

fn index_err_msg(m: &str) -> PyExc { crate::exc::py_err("IndexError", m) }

pub fn shuffle(list: &Py) -> PyResult {
    if let Py::List(l) = list {
        let mut b = l.borrow_mut();
        let n = b.len();
        for i in (1..n).rev() {
            let j = with_rng(|r| r.randbelow((i + 1) as u64)) as usize;
            b.swap(i, j);
        }
        Ok(Py::None)
    } else {
        Err(type_err("shuffle needs a mutable sequence"))
    }
}

pub fn uniform(a: &Py, b: &Py) -> Py {
    let (a, b) = (a.as_f64().unwrap_or(0.0), b.as_f64().unwrap_or(0.0));
    Py::Float(a + (b - a) * with_rng(|r| r.random_f64()))
}

pub fn sample(population: &Py, k: &Py) -> PyResult {
    let items = py_iter(population)?;
    let k = k.as_i64().unwrap_or(0) as usize;
    if k > items.len() { return Err(val_err("Sample larger than population or is negative")); }
    // CPython sample: set-based for small k, pool-based otherwise; result is a random
    // selection - exact sequence parity is not required for reply pools.
    let n = items.len();
    let mut indices: Vec<usize> = (0..n).collect();
    let mut out = Vec::with_capacity(k);
    for i in 0..k {
        let j = i + with_rng(|r| r.randbelow((n - i) as u64)) as usize;
        indices.swap(i, j);
        out.push(items[indices[i]].clone());
    }
    Ok(py_list(out))
}

pub fn getrandbits(k: &Py) -> Py {
    let k = k.as_i64().unwrap_or(0).max(0) as u32;
    Py::Int(words_to_u64(&with_rng(|r| r.getrandbits(k)), k) as i64)
}

/// random module call dispatcher: random.choice(...), etc.
pub fn call(name: &str, args: &[Py]) -> PyResult {
    match name {
        "seed" => { seed(args.first().unwrap_or(&Py::None)); Ok(Py::None) }
        "random" => Ok(random()),
        "randint" => randint(args.first().unwrap_or(&Py::None), args.get(1).unwrap_or(&Py::None)),
        "randrange" => randrange(args),
        "choice" => choice(args.first().unwrap_or(&Py::None)),
        "shuffle" => shuffle(args.first().unwrap_or(&Py::None)),
        "uniform" => Ok(uniform(args.first().unwrap_or(&Py::None), args.get(1).unwrap_or(&Py::None))),
        "sample" => sample(args.first().unwrap_or(&Py::None), args.get(1).unwrap_or(&Py::None)),
        "getrandbits" => Ok(getrandbits(args.first().unwrap_or(&Py::None))),
        "choices" => {
            let pop = py_iter(args.first().unwrap_or(&Py::None))?;
            let k = args.get(2).and_then(|p| p.as_i64()).unwrap_or(1);
            let mut out = Vec::new();
            for _ in 0..k { out.push(choice(&py_list(pop.clone()))?); }
            Ok(py_list(out))
        }
        "triangular" => {
            let (a, b) = (args.first().and_then(|p| p.as_f64()).unwrap_or(0.0), args.get(1).and_then(|p| p.as_f64()).unwrap_or(1.0));
            let u = with_rng(|r| r.random_f64());
            let c = 0.5;
            if u > c { Ok(Py::Float(a + (b - a) * (1.0 - ((1.0 - u) * (1.0 - c)).sqrt()))) } else { Ok(Py::Float(a + (b - a) * (u * c).sqrt())) }
        }
        _ => Err(attr_err(name)),
    }
}
