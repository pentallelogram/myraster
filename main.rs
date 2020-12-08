// bug when x1 and x0 are same, loop won't exec (and would div by 0 if it did)
// todo 
//    1. actually set pixel on image (need to make classes in rust)
use std::mem::swap;
use std::{thread, time};

// Bresenham’s Line Drawing Algorithm
fn line(mut x0: u32, mut y0: u32, mut  x1: u32, mut y1: u32) {

    println!("\ncase: x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", x0, y0, x1, y1);

    let mut steep: bool = false;
    let mut sx0: i64 = x0.into();
    let mut sy0: i64 = y0.into();
    let mut sx1: i64 = x1.into();
    let mut sy1: i64 = y1.into();
    let xdist = (sx0-sx1).abs();
    let ydist = (sy0-sy1).abs();
    if ydist > xdist {
        steep = true;
    }
    println!("xdist: {}  {} ydist: {} so line is{} steep",
        xdist, if steep { "<" } else { ">"}, 
        ydist, if steep { "" } else {"n't"});
    let looplen: i64 = if steep { ydist } else { xdist };

    if y0 > y1 {
        // make all ray's point down or sideways not up
        println!("swap points!");
        println!("x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", sx0, sy0, sx1, sy1);
        swap(&mut sx0, &mut sx1);
        swap(&mut sy0, &mut sy1);
        println!("x0: {}\ty0: {} \t&& \tx1: {}\ty1: {}", sx0, sy0, sx1, sy1);
    }

    for n in 0..looplen {
        //println!("y0:{} y1:{}", y0, y1);
        let sx: i64 = sx0 + (sx1-sx0) * n /looplen;
        let sy: i64 = sy0 + (sy1-sy0) * n /looplen;
        println!("{}:\t{}\t{}", n, sx, sy);
        // set pixel at (x,y) to color
    }

    return;
}

fn main() {
    let t = time::Duration::from_secs(2);

    // down to the left
    line( 0, 0, 20, 20); thread::sleep(t);

    // up to the right
    line( 40, 40, 20, 20); thread::sleep(t);

    // down to the left
    line( 40, 0, 20, 20); thread::sleep(t);

    // up to the right
    line( 0, 40, 20, 20); thread::sleep(t);

    return;
}
