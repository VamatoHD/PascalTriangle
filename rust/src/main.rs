mod render;
use render::render;

mod pascal;
use pascal::*;

use num_bigint::BigUint;

fn measure<F: Fn(u32) -> Vec<BigUint>>(f: F, times: usize, max: f32) -> Vec<(f32, f32)> {
    use pbr::ProgressBar;
    use std::time::Instant;

    let mut res = vec![];

    let mut pb = ProgressBar::new(1000);
    pb.format("[=>-]");

    for i in 1..=1000 {
        let test = i as u32 * 100;

        let mut total = 0;
        for _ in 1..=times {
            let start = Instant::now();
            let _ = f(test);
            let end = start.elapsed().as_millis();
            total += end;

            pb.inc();
        }

        let t = (total / (times as u128)) as f32;
        res.push((test as f32, t));

        if t > max {
            pb.finish_print("Finished early.");
            return res;
        }
    }
    pb.finish_print("Finished.");
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = measure(pascal1, 5, 3.0 * 1000.0);

    render(&vec![a], "output.png")?;

    Ok(())
}
