use rand::Rng;

fn main() {
    let mut hailstone: u32 = rand::thread_rng().gen_range(1..=100);
    let mut iters: u32 = 0;
    loop {
        println!("{hailstone}");
        iters = iters + 1;
        if hailstone == 1 {
            println!("DONE: took {iters} iterations");
            break;
        }
        else if hailstone % 2 == 0 {
            hailstone = hailstone / 2;
        } else {
            hailstone = 3 * hailstone + 1;
        }
    }
}
