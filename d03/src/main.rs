use nom::{
    bytes::complete::tag,
    character::complete::{char, i32},
    IResult,
};
use std::time::Instant;

fn mul(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, n1) = i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, n2) = i32(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, (n1, n2)))
}

fn p1(input: &str) -> i32 {
    let mut sum = 0;
    let mut rem = input;
    while !rem.is_empty() {
        if let Ok((new_rem, (a, b))) = mul(rem) {
            sum += a * b;
            rem = new_rem;
        } else {
            rem = &rem[1..];
        }
    }
    sum
}

fn on(input: &str) -> IResult<&str, &str> {
    tag("do()")(input)
}

fn off(input: &str) -> IResult<&str, &str> {
    tag("don't()")(input)
}

fn p2(input: &str) -> i32 {
    let mut sum = 0;
    let mut enabled = 1;
    let mut rem = input;
    while !rem.is_empty() {
        if let Ok((input, _)) = on(rem) {
            enabled = 1;
            rem = input;
        } else if let Ok((input, _)) = off(rem) {
            enabled = 0;
            rem = input;
        } else if let Ok((input, (a, b))) = mul(rem) {
            sum += a * b * enabled;
            rem = input;
        } else {
            rem = &rem[1..];
        }
    }
    sum
}

fn main() {
    let content = include_str!("input.txt");
    let time = Instant::now();
    let p1 = p1(content);
    let p2 = p2(content);
    println!("Time: {}µs", time.elapsed().as_micros());
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}