/// Rusty Donut
use std::{thread, time};

fn main() {
    let mut a_cap: f32 = 0.0;
    let mut b_cap: f32 = 0.0;
    let mut i: f32;
    let mut j: f32;
    let mut z: [f32; 1760] = [0.0f32; 1760];
    let mut b: [char; 1760] = [' '; 1760];

    print!("\x1b[2J");

    loop {
        b.fill(' ');
        z.fill(0.0);

        j = 0.0;
        while j < 6.28 {
            i = 0.0;
            while i < 6.28 {
                let c: f32 = f32::sin(i);
                let d: f32 = f32::cos(j);
                let e: f32 = f32::sin(a_cap);
                let f: f32 = f32::sin(j);
                let g: f32 = f32::cos(a_cap);
                let h: f32 = d + 2.0;
                let d_cap: f32 = 1.0 / (c * h * e + f * g + 5.0);
                let l: f32 = f32::cos(i);
                let m: f32 = f32::cos(b_cap);
                let n: f32 = f32::sin(b_cap);
                let t: f32 = c * h * g - f * e;

                let x: i32 = (40.0 + 30.0 * d_cap * (l * h * m - t * n)) as i32;
                let y: i32 = (12.0 + 15.0 * d_cap * (l * h * n + t * m)) as i32;

                let o: i32 = x + 80 * y;
                let n_cap: i32 =
                    (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as i32;

                if 22 > y && y > 0 && x > 0 && 80 > x && d_cap > z[o as usize] {
                    z[o as usize] = d_cap;
                    b[o as usize] = ".,-~:;=!*#$@"
                        .chars()
                        .nth((if n_cap > 0 { n_cap } else { 0 }) as usize)
                        .unwrap();
                }

                i += 0.02
            }

            j += 0.07;
        }

        print!("\x1b[H");

        for k in 0..=1760 {
            let c: char = if (k % 80) != 0 { b[k] } else { '\n' };
            print!("{c}");
            a_cap += 0.00004;
            b_cap += 0.00002;
        }

        let loop_period = time::Duration::from_micros(30000);
        thread::sleep(loop_period);
    }
}

#[cfg(test)]
mod test;
