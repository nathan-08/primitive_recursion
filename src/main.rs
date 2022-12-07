use static_assertions::const_assert;
type N = usize;

macro_rules! id {
    ($n:literal, $m:literal) => {{
        const_assert! ($m > 0);
        const_assert! ($m <= $n);
        |arr : [usize; $n]| {
            arr[$m - 1]
        }
    }}
}

macro_rules! pr {
    ($f:ident, $g:ident, $h:ident) => { // f = Pr[g, h]
        fn $f (n : N, x : N) -> N {
            match n {
                0 => $g(x),
                n => $h(n - 1, x, $f(n - 1, x))
            }
        }
    };
}

const ZERO : N = 0;
fn suc (n : N) -> N { n + 1 }

fn pr_add_g (x : N) -> N { x } // base case
fn pr_add_h (_n : N, _x : N, rec_call : N) -> N { suc(rec_call) } // recursive case
pr! (pr_add, pr_add_g, pr_add_h); // construct pr function called pr_add

fn pr_mult_g (_x : N) -> N { ZERO } // base case
fn pr_mult_h (_n : N, m : N, rec_call : N) -> N { pr_add(m, rec_call) } // recursive case
pr! (pr_mult, pr_mult_g, pr_mult_h); // construct pr function called pr_mult

fn pr_exp_g (_x : N) -> N { suc(ZERO) } // base case
fn pr_exp_h (_n : N, m : N, rec_call : N) -> N { pr_mult(m, rec_call) } // recursive case
pr! (pr_exp, pr_exp_g, pr_exp_h); // pr_exp(n, m) = m^n

fn main() {
    println!("{}", pr_exp( 5, 2 ));  // 2^5 = 32
}

