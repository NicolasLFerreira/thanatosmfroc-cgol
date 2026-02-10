use clap::Parser;
use egui::IntoAtoms;
use thanatos::persistence::Database;
use thanatos::startup::{StartupParameters, startup};
use thanatos::utilities::bit_packing::unpack_u64_u32;

#[derive(Parser)]
struct Cli {
    // No GUI
    #[arg(long, default_value_t = false)]
    headless: bool,
    // Run cap
    #[arg(long, default_value_t = 100)]
    max_runs: u32,
    // Generation cap per run
    #[arg(long, default_value_t = 1_000)]
    max_generations: u32,
}

fn main() {
    // let db = Database::open();
    // let mut hash: u128 = 622177874904329648928522284728368885;
    // 
    // for _ in 0..200 {
    //     if let Some(v) = db.get(hash) {
    //         let mut s = v
    //             .configuration
    //             .iter()
    //             .map(|c| unpack_u64_u32(*c))
    //             .collect::<Vec<_>>();
    //         s.sort_by(|(x1, y1), (x2, y2)| x1.cmp(x2).then(y1.cmp(y2)));
    //         println!("{:?}", s);
    //         hash = v.next_hash;
    //     } else {
    //         break;
    //     }
    // }
    // 
    // return;
    let cli = Cli::parse();

    // build startup
    let startup_params = StartupParameters {
        max_runs: cli.max_runs,
        max_generations: cli.max_generations,
        run_headless: cli.headless,
    };

    startup(startup_params);
}
