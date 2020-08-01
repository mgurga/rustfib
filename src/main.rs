
const FIBLENGTH: u32 = 20;

fn main() {
    let mut fiblist: [i128; FIBLENGTH as usize + 2] = [-1; FIBLENGTH as usize + 2];
    fiblist[0] = 0;
    fiblist[1] = 1;

    println!("0: {} {}", fiblist[0], fiblist[1]);
    
    for i in 1..=FIBLENGTH {
        let newnum = fiblist.iter().position(|&x| x == -1).unwrap();

        fiblist[newnum] = fiblist[newnum - 1] + fiblist[newnum - 2];

        print!("{}: ", i);
        for j in fiblist.iter() {
            if *j != -1 {
                print!("{} ", j);
            }
        }
        println!();
    }
}