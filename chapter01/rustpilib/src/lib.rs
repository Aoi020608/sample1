use rand::prelude::*;

pub fn monte_carlo_pi(iterations: usize) -> f32 {
    let mut inside_circle = 0;
    for _ in 0..iterations {
        // generate two random coordinates between 0 and 1
        let x: f32 = random::<f32>();
        let y: f32 = random::<f32>();
        // calculate the circular distance from 0, 0
        if x.powi(2) + y.powi(2) <= 1_f32 {
            // if it's within the circle, increase the count
            inside_circle += 1;
        }
    }

    // return the ratio of 4 times the hits to the total
    (4_f32 * inside_circle as f32) / iterations as f32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
