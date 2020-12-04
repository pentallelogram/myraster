// bug when x1 and x0 are same, loop won't exec (and would div by 0 if it did)
// todo 
//    1. actually set pixel on image (need to make classes in rust)
//    2. make it not need to go from top left to bottom right
//    3. make it not only write 3 points when passing (1,1,4,80) for example
use std::mem::swap;

// Bresenham’s Line Drawing Algorithm
fn line(mut x0: u32, mut y0: u32, mut  x1: u32, mut y1: u32) {


    let mut steep: bool = false;
    let sx0: i64 = x0.into();
    let sy0: i64 = y0.into();
    let sx1: i64 = x1.into();
    let sy1: i64 = y1.into();
    let xdist = (sx0-sx1).abs();
    let ydist = (sy0-sy1).abs();
    if ydist > xdist {
        steep = true;
    }
    println!("xdist: {}  {} ydist: {} so line is{} steep",
        xdist, if steep { "<" } else { ">"}, 
        ydist, if steep { "" } else {"n't"});
    let looplen: u32 = if steep { ydist as u32 }
                    else { xdist as u32 };
    println!("looplen: {} (number of pixels to color)", looplen);

/* BUG: my transpose isn't working when vectors point ↗️  or ↙️ .
    if (x0 > x1 && y0 < y1) || (x1 > x0 && y1 < y0) {
        println!("transpose!");
        println!("x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", x0, y0, x1, y1);
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        println!("x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", x0, y0, x1, y1);
        transpose = true;
    }
*/

    if x0 > x1 {
        println!("swap points!");
        println!("x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", x0, y0, x1, y1);
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
        println!("x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", x0, y0, x1, y1);
    }


    for n in 0..looplen {

        let x: u32 = x0 + (x1-x0) * n * 1/looplen;
        let y: u32 = y0 + (y1-y0) * n * 1/looplen;

        if steep {
            println!("{}:\t{}\t{}", n, y, x);
        } else {
            println!("{}:\t{}\t{}", n, x, y);
        }
        // set pixel at (x,y) to color
    }

    return;
}

fn main() {
    line( 0, 40, 20, 20);
    return;
}
