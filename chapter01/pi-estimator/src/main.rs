// declare the module by its file name
mod rounding;
mod example;

// Rust will also accept if you implement it right away
mod printer {
    // import a function from external crate (no more extern declaration required!)
    use rustpilib::monte_carlo_pi;

    // crates present in the parent can be imported using the crate prefix
    use crate::rounding::round;

    // internal crates can always be imported using the crate
    pub fn pretty_print_pi_approx(iterations: usize) {
        let pi = monte_carlo_pi(iterations);
        let places: usize = 2;
        println!(
            "Pi is ~ {} and rounded to {} places {}",
            pi,
            places,
            round(pi, places)
        );
    }
}

// import from the module above
use printer::pretty_print_pi_approx;

fn main() {
    pretty_print_pi_approx(100_000);
}
