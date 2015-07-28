#![feature(iter_arith)]     // for .sum()
                            // unstable library feature 'iter_arith' : bounds recently changed

fn p1_sol1() -> u64 {
    // formula solution

    //  3,  6,  9, ...
    //  5, 10, 15, ...
    // 15, 30, 45, ...

    //  3 * 1,  3 * 2,  3 * 3, ...,  3 * a => sum:  3 * a * (1 + a) / 2
    //  5 * 1,  5 * 2,  5 * 3, ...,  5 * b => sum:  5 * b * (1 + b) / 2
    // 15 * 1, 15 * 2, 15 * 3, ..., 15 * c => sum: 15 * c * (1 + c) / 2

    fn sum_range(start: u64, end: u64) -> u64 {
        end * (start + end) / 2
    }

    fn sum_multiples_of_two_below(base_num: &[u64], end: u64) -> u64 {
        let v1 = base_num[0];
        let v2 = base_num[1];
        let a = (end - 1) / v1;
        let b = (end - 1) / v2;
        let c = (end - 1) / (v1 * v2);
        let sum_a = v1 * sum_range(1, a);
        let sum_b = v2 * sum_range(1, b);
        let sum_c = (v1 * v2) * sum_range(1, c);
        sum_a - sum_c + sum_b
    }

    sum_multiples_of_two_below(&[3, 5], 1000)

    // [TODO]
    // make function more general
    // make sure the array's elements is unique (no repeat)
    // generate combinations (for more base numbers)

    // -C opt-level=3 :
    //
    //      p1_sol1::h7b3c1cda8687c063eaa:
    //          cmpq     %fs:112, %rsp
    //          ja       .LBB0_2
    //          movabsq  $8, %r10
    //          movabsq  $0, %r11
    //          callq    __morestack
    //          retq
    //      .LBB0_2:
    //          pushq    %rbp
    //          movq     %rsp, %rbp
    //          movl     $233168, %eax
    //          popq     %rbp
    //          retq
    //
    // -C opt-level=3 -C no-stack-check :
    //
    //      p1_sol1::h7b3c1cda8687c063eaa:
    //          pushq   %rbp
    //          movq    %rsp, %rbp
    //          movl    $233168, %eax
    //          popq    %rbp
    //          retq
}

fn p1_sol2() -> u64 {
    // -C opt-level=3 => inline to main (not calculate at compile time)
    // simple filter solution
    (1..1000)
        .filter(|&n| (n % 3 == 0) || (n % 5 == 0))
        .sum()
}

fn p1_sol3() -> u64 {
    // general filter solution
    fn sum_multiples_of_base_below(base_num: &[u64], start: u64, end: u64) -> u64 {
        (start..end)
            .filter(|&n| base_num.iter().any(|&base| n % base == 0))
            .sum()
    }

    sum_multiples_of_base_below(&[3, 5], 1, 1000)
}

fn p1_sol4() -> u64 {
    // macro version for filter
    macro_rules! sum_multiples_of_base_below {
        ( OωO $($base:expr),+ OωO , $start:expr, $end:expr ) => {
            ($start..$end)
                .filter(|&n| $(n % $base == 0)||+)
                .sum()
        }
    }

    sum_multiples_of_base_below!(OωO 3, 5 OωO, 1, 1000)
}

fn p1_sol5() -> u64 {
    // closure

    let mut s = 0;
    let mut v = 0;

    {
        // only borrow in this scope

        let mut f = || {
            v = v + 1;
            s = s + ((v % 3) * (v % 5) < 1) as u64 * v;
        };

        for _ in 1..1000 {
            f();
        }
    }

    s
}

fn p1_sol6() -> u64 {
    // static variable

    fn f() -> u64 {
        static mut s: u64 = 0;
        static mut v: u64 = 0;

        // static mut is unsafe
        unsafe {
            v = v + 1;
            s = s + ((v % 3) * (v % 5) < 1) as u64 * v;

            s
        }
    }

    let mut result = 0;

    for _ in 1..1000 {
        result = f();
    }

    result
}

fn p1_sol7() -> u64 {
    // iterator

    struct Euler {
        s: u64,
        v: u64,
    }

    impl Iterator for Euler {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let s = self.s;
            let v = self.v;
            let s = s + ((v % 3) * (v % 5) < 1) as u64 * v;
            let v = v + 1;
            self.s = s;
            self.v = v;

            Some(self.s)
        }
    }

    let euler = Euler { s: 0, v: 0 };

    euler.skip(1000-1)
         .next().unwrap_or(0)
}

fn p1_sol8() -> u64 {
    // overload index
    // index has side-effect, it's bad :P

    use std::ops::{Index, IndexMut};

    struct Euler {
        s: u64,
        v: u64,
    }

    impl Index<u64> for Euler {
        type Output = u64;

        fn index(&self, _index: u64) -> &u64 {
            &self.s
        }
    }

    impl IndexMut<u64> for Euler {
        fn index_mut(&mut self, _index: u64) -> &mut u64 {
            let v = self.v;
            let s = self.s;
            let v = v + 1;
            let s = s + ((v % 3) * (v % 5) < 1) as u64 * v;
            self.v = v;
            self.s = s;
            &mut self.s
        }
    }

    let mut euler = Euler { s: 0, v: 0 };

    for _ in 1..1000 {
        &mut euler[0];
    }

    euler[0]
}

fn main() {
    // sum of all the multiples of 3 or 5 below 1000
    // ans : 233168
    println!("p1_sol1 : {}", p1_sol1());
    println!("p1_sol2 : {}", p1_sol2());
    println!("p1_sol3 : {}", p1_sol3());
    println!("p1_sol4 : {}", p1_sol4());
    println!("p1_sol5 : {}", p1_sol5());
    println!("p1_sol6 : {}", p1_sol6());
    println!("p1_sol7 : {}", p1_sol7());
    println!("p1_sol8 : {}", p1_sol8());
}
