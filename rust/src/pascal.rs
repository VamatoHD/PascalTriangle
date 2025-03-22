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
