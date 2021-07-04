//#[feature(trace_macros)]
use pattern_gen::{gen, Snap};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sm-pattern-gen",
    about = "Program for generating .sm file format streams of a given snap."
)]
struct Opt {
    snap: Snap,
}
fn main() {
    //trace_macros!(true);
    let opt = Opt::from_args();
    gen::gen_pattern(opt.snap);
}
