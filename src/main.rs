use crossbeam::atomic::AtomicCell;
use eframe::Renderer;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tmfroc::conway::simulation::*;
use tmfroc::thanatos;
use tmfroc::types::cell_configuration::CellConfiguration;
use tmfroc::types::cell_coord::CellCoord;
use tmfroc::ui::app::App;

fn main() {
    let glider: Vec<CellCoord> = vec![
        CellCoord::new(0, 0),
        CellCoord::new(0, 1),
        CellCoord::new(0, 2),
        CellCoord::new(1, 2),
        CellCoord::new(2, 1),
    ];

    let seed_cells = CellConfiguration::random_configuration(42, 20, 20, 0.3);
    let shared = Arc::new(AtomicCell::new(Arc::new(CellConfiguration::new())));

    // Simulation thread
    {
        let clone = Arc::clone(&shared);
        thread::spawn(move || run_logic(glider, clone))
    };

    run_ui(shared);
}

fn run_logic(seed_cells: Vec<CellCoord>, shared: Arc<AtomicCell<Arc<CellConfiguration>>>) {
    // Own the mutable working copy of the simulation state
    let mut cconf = CellConfiguration::with_seed_configuration(seed_cells);

    loop {
        let mut start = Instant::now();

        // Run Thanatos on current configuration
        thanatos::tmfroc::run(&cconf);
        let elapsed = start.elapsed();
        println!("Thanatos: {:?}", elapsed);

        // Step Conway simulation
        start = Instant::now();
        let new_cconf = simulation(&cconf);
        cconf = new_cconf;
        let elapsed = start.elapsed();
        println!("Simulation: {:?}", elapsed);

        // Publish a snapshot to UI (or other observers)
        // Only the Arc is cloned, not the entire HashSet
        shared.store(Arc::new(cconf.clone()));
        thread::sleep(Duration::from_millis(500));
    }
}

fn run_ui(shared: Arc<AtomicCell<Arc<CellConfiguration>>>) {
    let native_options = eframe::NativeOptions {
        centered: true,
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Thanatos CGoL",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, shared)))),
    )
    .unwrap();
}
