use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 50;
const CELL_SIZE: f32 = 16f32;
const TICK_DURATION: f32 = 0.2;

type Grid = Vec<bool>;
type Coord = (usize, usize);

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Game of Life"),
        window_width: CELL_SIZE as i32 * GRID_WIDTH as i32,
        window_height: CELL_SIZE as i32 * GRID_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cells: Grid = vec![false; GRID_WIDTH * GRID_HEIGHT];

    // Seeding
    let glider: Vec<Vec<bool>> = vec![
        vec![false, false, true],
        vec![true, false, true],
        vec![false, true, true],
    ];
    let shape = shape_translate(glider.clone(), 16, 24);
    seed(&mut cells, shape);
    let shape = shape_translate(glider.clone(), 20, 28);
    seed(&mut cells, shape);

    // Simulation
    let mut tick_timer = 0.0f32;
    loop {
        let dt = get_frame_time();
        tick_timer += dt;

        clear_background(WHITE);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if cells[idx(y, x)] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        BLACK,
                    );
                }
            }
        }

        // cap tick rate
        if tick_timer >= TICK_DURATION {
            simulation(&mut cells);
            tick_timer = 0.0;
        }

        next_frame().await;
    }
}

fn seed(cells: &mut Grid, coords: Vec<Coord>) {
    for (y, x) in coords {
        cells[idx(y, x)] = true;
    }
}

fn shape_translate(shape: Vec<Vec<bool>>, ay: usize, ax: usize) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for y in 0..shape.len() {
        for x in 0..shape[y].len() {
            if shape[y][x] {
                coords.push((y + ay, x + ax));
            }
        }
    }

    coords
}

// RULES:
// 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// 2. Any live cell with two or three live neighbours lives on to the next generation.
// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
fn simulation(cells: &mut Grid) {
    let current_state = cells.clone();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = idx(y, x);
            let is_alive = current_state[index];
            let alive_neighbours = neighbours((y, x), &current_state)
                .iter()
                .filter(|&&(ny, nx)| current_state[idx(ny, nx)])
                .count();

            // Applies rule
            cells[index] = match (is_alive, alive_neighbours) {
                (true, 2..=3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false,
            };
        }
    }
}

fn neighbours(coord: Coord, cells: &Grid) -> Vec<Coord> {
    let (y, x) = coord;
    let y = y as i32;
    let x = x as i32;

    let mut neighbours: Vec<Coord> = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            // Skip center
            if dy == 0 && dx == 0 {
                continue;
            }

            // Bound check
            let ny = y + dy;
            let nx = x + dx;
            if ny < 0 || ny >= GRID_HEIGHT as i32 {
                continue;
            }
            if nx < 0 || nx >= GRID_WIDTH as i32 {
                continue;
            }

            neighbours.push((ny as usize, nx as usize))
        }
    }

    neighbours
}

#[inline]
// Row major
fn idx(y: usize, x: usize) -> usize {
    y * GRID_WIDTH + x
}
