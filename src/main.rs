use num::{Integer, Zero};
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::One;
use std::ops::{Sub, AddAssign};
use text_io::read;

fn miller_rabin(num_ori: &BigUint, k: i16) -> bool {
    let num = num_ori.clone();

    let prev_num = num.clone().sub(1u8);

    if num <= BigUint::one() || num.is_even() {
        return false;
    }

    if num == BigUint::from(2u8) || num == BigUint::from(3u8) {
        return true;
    }

    let mut rng = rand::thread_rng();

    let mut power: u32 = 1;

    while prev_num.clone() % 2u128.pow(power + 1) == BigUint::zero() {
        power += 1;
    }

    let two = BigUint::from(2u8);

    let residue = (prev_num.clone() / two.pow(power)).to_biguint().unwrap();
    let num_big = num.to_biguint().unwrap();
    let num_sub_two = num.clone().sub(2u8);

    'tests: for _ in 0..k {
        let random = rng.gen_biguint_range(&BigUint::from(2u8), &num_sub_two);
        let mut x = random.modpow(&residue, &num_big);
        if x == 1.to_biguint().unwrap() || x == prev_num {
            continue 'tests;
        }
        for _ in 1..=power - 1 {
            x = x.modpow(&2.to_biguint().unwrap(), &num_big);
            if x == prev_num.to_biguint().unwrap() {
                continue 'tests;
            }
        }
        return false;
    }

    return true;
}
fn main() {
    println!("What is the number you're looking to check if it's a prime?");
    let mut input: String = read!();
    let mut num = input.parse::<BigUint>().expect("Error?");
    input.clear();

    println!("How many tests shall we run?");
    input = read!();
    let k: i16 = input.parse::<i16>().expect("Error?");

    while !miller_rabin(&num, k) {
        num.add_assign(1u8);
    }

    //println!("Result: {}", miller_rabin(&num, k));
    println!("Result: {}", num.to_string());
}
