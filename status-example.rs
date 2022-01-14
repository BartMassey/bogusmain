use status::*;

fn main() -> Status {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.len() {
        0 => {
            status!(1, "usage: main <arg>")
        }
        1 => {
            println!("{}", args[0]);
            status!(0)
        }
        2 => {
            status!(-1, &args[1])
        }
        _ => {
            status!(-1, "unexpected argument {}", args[2])
        }
    }
}
