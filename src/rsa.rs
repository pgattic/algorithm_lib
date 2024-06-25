use num_bigint::BigInt;

pub struct RSA;

impl RSA {
    pub fn euclid(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        if *b == BigInt::from(0) {
            return (a.clone(), BigInt::from(1), BigInt::from(0));
        }
        let (gcd, i, j) = Self::euclid(&b, &(a % b));
        return (gcd, j.clone(), i-(a/b) * j);
    }

    fn mod_inverse(i: &BigInt, b: &BigInt) -> BigInt {
        ((i % b) + b) % b
    }

    //fn modular_exponentiation(x: &BigInt, y: &BigInt, n: &BigInt) -> BigInt {
    //    if *y == BigInt::from(0) {
    //        return BigInt::from(1);
    //    } else if y % BigInt::from(2) == BigInt::from(0) {
    //        let z = Self::modular_exponentiation(x, &(y/2), n);
    //        return z.pow(2) % n
    //    }
    //    let z = Self::modular_exponentiation(x, &(y-1/2), n);
    //    (z.pow(2) * x) % n
    //}

    pub fn generate_private_key(p: &BigInt, q: &BigInt, e: &BigInt) -> BigInt {
        let phi = (p-1) * (q-1);
        let (_gcd, i, _j) = Self::euclid(e, &phi);
        //return i % phi
        //i.modinv(&phi).unwrap()
        Self::mod_inverse(&i, &phi)
    }

    pub fn encrypt(data: &BigInt, e: &BigInt, n: &BigInt) -> BigInt {
        //return data ** e % n;
        data.modpow(e, n)
        //Self::modular_exponentiation(data, e, n)
    }

    pub fn decrypt(data: &BigInt, d: &BigInt, n: &BigInt) -> BigInt {
        //return data ** d % n;
        data.modpow(d, n)
        //Self::modular_exponentiation(data, d, n)
    }
}

