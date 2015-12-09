use std::env;

fn usage(argv_0: &String) {
    println!("-----USAGE----");
    println!("{} project deploy             deploy project with latest <head>.", argv_0);
    println!("{}  project rollback <head>    rollback with <head>.", argv_0);
    std::process::exit(0);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        usage(&args[0]);
    }
}
