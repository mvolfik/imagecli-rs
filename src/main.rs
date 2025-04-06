use imagecli::{documentation::generate_guide, process};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opt {
    /// Enable verbose logging.
    #[arg(short, long)]
    verbose: bool,

    /// Input files or glob patterns.
    #[arg(short, long)]
    input: Vec<String>,

    /// Output files.
    #[arg(short, long)]
    output: Vec<String>,

    /// Image processing pipeline to apply.
    #[arg(short, long)]
    pipeline: Option<String>,

    /// Ignore all the other flags and regenerate a user guide instead.
    #[arg(long)]
    generate_guide: bool,
}

fn main() {
    let opt = Opt::parse();

    let result = if opt.generate_guide {
        generate_guide(opt.verbose)
    } else {
        process(&opt.input, &opt.output, opt.pipeline, opt.verbose)
    };

    match result {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
