// bug when x1 and x0 are same, loop won't exec (and would div by 0 if it did)
// todo 
//    1. actually set pixel on image (need to make classes in rust)
//    2. make it not need to go from top left to bottom right
//    3. make it not only write 3 points when passing (1,1,4,80) for example

// Bresenham’s Line Drawing Algorithm
fn line(x0: u32, y0: u32, x1: u32, y1: u32) {

    // make 100 points between (x0, y0) and (x1, y1)
    let len = x1-x0;
    for n in x0..x1 {

        let x: u32 = x0 + (x1-x0) * n * 1/len;
        let y: u32 = y0 + (y1-y0) * n * 1/len;

        println!("{}:\t{}\t{}", n, x, y);

        // set pixel at (x,y) to color
    }

    return;
}

fn main() {
    line( 1, 1, 4, 80);
    println!("hello world");
}
