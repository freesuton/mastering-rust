use clap::Parser;

// Create your command-line parser, with all of the bells and whistles, declaratively or procedurally.
// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Name of the person to greet
    #[clap(short, long)]
    name: String,

    // Number of times to greet
    #[clap( long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

}

//https://www.fpcomplete.com/rust/command-line-parsing-clap/
//with long key word you can use --xx xx to set value
// cargo run -- --name XXX --count 2
//with short key word you can use -x xx to set value