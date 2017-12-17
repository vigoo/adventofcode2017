use std::collections::VecDeque;

pub fn gen_a1(prev: i64) -> i64 {
    return (prev * 16807) % 2147483647;
}

pub fn gen_b1(prev: i64) -> i64 {
    return (prev * 48271) % 2147483647;
}

pub fn part1() {
    let mut a: i64 = 783;
    let mut b: i64 = 325;
    let mut matches: i32 = 0;

    for _i in 0..40000000 {
        a = gen_a1(a);
        b = gen_b1(b);

        let a_lo = a & 0xffff;
        let b_lo = b & 0xffff;

        if a_lo == b_lo {
            matches = matches + 1;
        }
    }

    println!("Day 15 result 1: {}", matches);
}

pub fn gen_a2(prev: i64) -> (i64, bool) {
    let next = (prev * 16807) % 2147483647;
    return (next, next % 4 == 0);
}

pub fn gen_b2(prev: i64) -> (i64, bool) {
    let next = (prev * 48271) % 2147483647;
    return (next, next % 8 == 0);
}

pub fn part2() {
    let mut a: i64 = 65;
    let mut a_check: VecDeque<i64> = VecDeque::new();
    let mut b: i64 = 8921;
    let mut b_check: VecDeque<i64> = VecDeque::new();

    for _i in 0..5000000 {
        let (a_next, a_ok) = gen_a2(a);
        let (b_next, b_ok) = gen_b2(b);

        a = a_next;
        b = b_next;

        if a_ok {
            a_check.push_back(a_next);
        }
        if b_ok {
            b_check.push_back(b_next);
        }
    }

    let mut matches: i32 = 0;
    let mut n = 0;

    while !a_check.is_empty() && !b_check.is_empty() {
        let a = a_check.pop_front().unwrap();
        let a_lo =  a & 0xffff;
        let b = b_check.pop_front().unwrap();
        let b_lo = b & 0xffff;

        if n < 10 {
            println!("{} {}", a, b);
            n = n + 1;
        }

        if a_lo == b_lo {
            matches = matches + 1;
        }
    }

    println!("Day 15 result 2: {}", matches);
}

pub fn run() {
    part1();
    part2();
}