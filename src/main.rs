extern crate argparse;
extern crate polynomial;

#[macro_use]
extern crate uint;

use uint::construct_uint;

use argparse::{ArgumentParser, Collect, Store, StoreTrue};
use polynomial::Polynomial;
use Vec;
use std::str::FromStr;


construct_uint! {
    pub struct U512(8);
}



fn main() {
    
    let mut verbose = false;
    let mut target_str: String = String::new();
    let mut coins: Vec<u64> = vec![1, 2, 3];
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Calculates the all the possible number of ways to make a certain amount of money with given coin values");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut target_str)
            .add_option(&["--target"], Store, "The number of cents to target");
        ap.refer(&mut coins)
            .add_option(&["--coist"], Collect, "Which coin values are available");
        ap.parse_args_or_exit();
    }
    println!("Targtet: \"{}\"", target_str);
    let target = U512::from_str(&target_str[..]).expect("Target must be a positive integer");

    //let coins = vec![1, 5, 10, 25, 50, 100];
    if verbose {
        println!("Using target cents {}", target);
        println!("Using coin values: {:?}", coins);
    }

    let mut coin_polys = Vec::new();

    let big_dollars = U512::from(target / 100 as u32);

    //Formulia from the video
    let result_count = (U512::from(80 as u32) * big_dollars * big_dollars * big_dollars * big_dollars * big_dollars
        + U512::from(390 as u32) * big_dollars * big_dollars * big_dollars * big_dollars
        + U512::from(672 as u32) * big_dollars * big_dollars * big_dollars
        + U512::from(483 as u32) * big_dollars * big_dollars
        + U512::from(127 as u32) * big_dollars
        + U512::from(6 as u32))
        / U512::from(6 as u32);

    if result_count < U512::from(50000000 as u32) {
        for coin in &coins {
            let mut coeffs = Vec::new();
            let mut value = 0;
            while U512::from(value) <= target {
                while (coeffs.len() as u64) < value {
                    coeffs.push(0);
                }
                coeffs.push(1);
                value += coin;
            }
            while U512::from(coeffs.len()) <= target {
                coeffs.push(0);
            }
            assert!(U512::from(coeffs.len() - 1) == target);
            coin_polys.push(Polynomial::<usize>::new(coeffs));
        }

        let mut result = Polynomial::<usize>::new(vec![1]);
        for poly in coin_polys {
            result = result * poly;
        }

        if verbose {
            println!("Result big poly: {:?}", result);
        }
        println!(
            "The coin values {:?} can be arranged {} ways to make {} cents. Obtained using the polynomial method",
            coins,
            result.data()[target.bits() as usize],
            target
        );
    } else if target % 100 as u32 == U512::from(0) {
        println!(
            "The coin values {:?} can be arranged {} ways to make {} cents. Obtained using the mathologer formulia",
            coins,
            result_count,
            target
        );
    } else {
        println!("Error unable to calculate exact result. Large values must be a mutiple of a dollor in order to work. Result =~{}", result_count);
    }
}



