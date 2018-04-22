#[macro_use]
extern crate clap;
extern crate rayon;
extern crate rand;
extern crate primal;
extern crate image;
extern crate num;

mod backends;
mod checkers;
mod extra_ops;

use std::error::Error as StdError;
use std::str;
use std::iter;
use std::fs::File;
use std::io::Write;

use rayon::prelude::*;
use num::traits::Num;
use num::integer::Integer;
use image::{GenericImage, Pixel, FilterType};

use backends::BigInt;
use extra_ops::ToStrRadix;
use checkers::PrimeChecker;
use checkers::SieveChecker;
use checkers::MillerRabinChecker;

type Error = Box<StdError>;

const MAX_ITERATIONS: usize = 1 << 32;

pub fn next_prime<N, C>(from: N, checker: &C) -> N 
    where N: Num + Integer + Clone + From<usize> + Send + Sync,
          C: PrimeChecker<Value=N> + Sync 
{
    let n = if from.is_even() {
        from + N::one()
    } else {
        from
    };

    (0..MAX_ITERATIONS).into_par_iter()
        .map(|i| {
            n.clone() + N::from(i * 2)
        })
        .find_any(|n| {
            checker.check(n)
        })
        .expect("Could not find prime number")
}

fn run() -> Result<(), Error> {
    let matches = clap_app!(primify =>
        (version: "0.1")
        (author: "0xd34d10cc")
        (about: "Generate prime number that in binary form looks like input image")
        (@arg INPUT:      -i --input     +takes_value +required "Input file name")
        (@arg OUTPUT:     -o --output    +takes_value "Output file name")
        (@arg ITER:       -n --witnesses +takes_value "Number of witnesses")
        (@arg SIEVE:      -s --sieve     +takes_value "Sieve upper bound")
        (@arg MAX_WIDTH:  -w --width     +takes_value "Max image width")
        (@arg MAX_HEIGTH: -h --height    +takes_value "Max image height")
        (@arg EDGE:       -e --edge      +takes_value "The color edge between 0 and 1 (0-255)")
    ).get_matches();
    
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap_or("output.txt");
    let edge = matches.value_of("EDGE")
        .map(|e| e.parse())
        .unwrap_or(Ok(128))?;
    let width: u32 = matches.value_of("MAX_WIDTH")
        .map(|w| w.parse())
        .unwrap_or(Ok(100))?;
    let height: u32 = matches.value_of("MAX_HEIGTH")
        .map(|h| h.parse())
        .unwrap_or(Ok(100))?;
    let sieve_size = matches.value_of("SIEVE")
        .map(|s| s.parse())
        .unwrap_or(Ok(8192))?;
    let witnessess = matches.value_of("ITER")
        .map(|i| i.parse())
        .unwrap_or(Ok(25))?;

    let image = image::open(input)?
        .resize(width, height, FilterType::Lanczos3)
        .grayscale();
    let image = image.resize_exact(image.width() * 2, image.height(), FilterType::Lanczos3);

    let width  = image.width()  as usize;
    let height = image.height() as usize;
    println!("Result image size is {}x{}", width, height);

    let npixels = width * height;
    let mut ascii_image = String::with_capacity(npixels + height);
    for (_, _, pixel) in image.pixels() {
        let (r, g, b, _) = pixel.to_rgb().channels4();
        let (r, g, b) = (r as u32, g as u32, b as u32);
        
        if r + g + b <= edge * 3 {
            ascii_image.push('1');
        } else {
            ascii_image.push('0');
        }
    }

    let nzeros = ascii_image.chars()
        .take_while(|&c| c == '0')
        .count();

    let num = BigInt::from_str_radix(&ascii_image[nzeros..], 2)?;
    println!("Number is {} bits long", npixels - nzeros);
    
    let checker = {
        let sieve = SieveChecker::new(sieve_size);
        let miller_rabin = MillerRabinChecker::new(witnessess);
        sieve.combine(miller_rabin)
    };
    let next_prime = next_prime(num, &checker);
    
    let bin = {
        let mut bin = String::with_capacity(npixels); 
        bin.extend(iter::repeat('0').take(nzeros));
        let prime = next_prime.to_str(2u8);
        bin.push_str(&prime);
        bin
    };

    ascii_image.clear();
    for line in bin.as_bytes().chunks(width) {
        let line = unsafe { str::from_utf8_unchecked(line) }; 
        ascii_image.push_str(line);
        ascii_image.push('\n');
    }

    let mut out = File::create(output)?;
    out.write_all(ascii_image.as_bytes())?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("An error occurred: {}", e);
    }
}