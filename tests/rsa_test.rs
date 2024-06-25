
mod rsa_test {
    use num_bigint::BigInt;
    use algorithm_lib::RSA;

    #[test]
    pub fn test_euclid_small() {
        let a = BigInt::from(8);
        let b = BigInt::from(12);

        let (gcd, i, j) = RSA::euclid(&a, &b);

        assert_eq!(gcd, BigInt::from(4));
        assert_eq!(i, BigInt::from(-1));
        assert_eq!(j, BigInt::from(1));
    }

    #[test]
    pub fn test_euclid_coprime_small() {
        let p = BigInt::from(7);
        let q = BigInt::from(13);
        let r = (p-1) * (q-1);
        let e = BigInt::from(5);
        let (gcd, i, j) = RSA::euclid(&e, &r);
        println!("{} {} {}", gcd, i, j);

        assert_eq!(gcd, BigInt::from(1));
        assert_eq!(i, BigInt::from(29));
        assert_eq!(j, BigInt::from(-2));
    }

    #[test]
    pub fn test_euclid_coprime_big() {
        let p = BigInt::from(87178291199 as i128);
        let q = BigInt::from(22815088913 as i128);
        let r = (p-1) * (q-1);
        let e = BigInt::from(65537);
        let (gcd, i, j) = RSA::euclid(&e, &r);
        println!("{} {} {}", gcd, i, j);

        assert_eq!(gcd, BigInt::from(1));
        assert_eq!(i, BigInt::from(-691197798001282429727 as i128));
        assert_eq!(j, BigInt::from(22775));
    }

    #[test]
    pub fn test_generate_private_key() {
        let p = BigInt::from(87178291199 as i128);
        let q = BigInt::from(22815088913 as i128);
        let e = BigInt::from(65537); // relatively prime to (p-1)*(q-1)

        let private_key = RSA::generate_private_key(&p, &q, &e);
        assert_eq!(private_key, BigInt::from(1297782666877314566849 as i128))
    }

    #[test]
    pub fn test_encrypt() {
        let p = BigInt::from(87178291199 as i128);
        let q = BigInt::from(22815088913 as i128);
        let e = BigInt::from(65537); // relatively prime to (p-1)*(q-1)
        let n = p.clone() * q.clone();
        let value = BigInt::from(42);
        //let private_key = RSA::generate_private_key(&p, &q, &e);
        let encrypted = RSA::encrypt(&value, &e, &n);
        assert_eq!(encrypted, BigInt::from(475967911669796538187 as i128));
    }

    #[test]
    pub fn test_decrypt() {
        let p = BigInt::from(87178291199 as i128);
        let q = BigInt::from(22815088913 as i128);
        let e = BigInt::from(65537); // relatively prime to (p-1)*(q-1)
        let n = p.clone() * q.clone();
        let value = BigInt::from(42);
        let private_key = RSA::generate_private_key(&p, &q, &e);
        let encrypted = RSA::encrypt(&value, &e, &n);
        let decrypted = RSA::decrypt(&encrypted, &private_key, &n);
        assert_eq!(decrypted, BigInt::from(42));
    }
}

