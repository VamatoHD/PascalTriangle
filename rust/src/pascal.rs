use num_bigint::BigUint;
use num_traits::One;

pub fn pascal1(n: u32) -> Vec<BigUint> {
    match n {
        1 => vec![BigUint::one()],
        2 => vec![BigUint::one(), BigUint::one()],
        _ => {
            let mut cur = vec![BigUint::one(), BigUint::one()];

            for _ in 2..n {
                let mut n = cur
                    .windows(2)
                    .map(|x| x[0].clone() + x[1].clone())
                    .collect::<Vec<BigUint>>();

                n.insert(0, BigUint::one());
                n.push(BigUint::one());

                cur = n;
            }
            cur
        }
    }
}

pub fn pascal2(n: u32) -> Vec<BigUint> {
    match n {
        1 => vec![BigUint::one()],
        2 => vec![BigUint::one(), BigUint::one()],
        _ => {
            let mut cur = Vec::with_capacity(n as usize);
            cur.push(BigUint::one());
            cur.push(BigUint::one());
            for _ in 2..n {
                for i in (1..cur.len()).rev() {
                    cur[i] = &cur[i] + &cur[i - 1]
                }
                cur.push(BigUint::one());
            }
            cur
        }
    }
}
