use std::io::BufReader;
use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize,Serialize};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use raylib::prelude::*;

#[derive(Serialize, Deserialize)]
struct SimulationResult {
    simulation_id: usize,
    values: Vec<SimulationFrame>,
}

#[derive(Serialize,Deserialize)]
struct SimulationFrame {
    tick: i32,
    income: i32,
    balance: i32,
    income_upgrade_cost: i32,
    income_upgrade_counts: i32
}



fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Simulation Visualization")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Draw axes
        d.draw_line(50, 550, 750, 550, Color::BLACK); // X-axis
        d.draw_line(50, 550, 50, 50, Color::BLACK);   // Y-axis

        // Draw simulation data
        for i in 1..simulation.values.len() {
            let x1 = 50 + (simulation.values[i - 1].tick / 10) as i32;
            let y1 = 550 - (simulation.values[i - 1].balance / 10);
            let x2 = 50 + (simulation.values[i].tick / 10) as i32;
            let y2 = 550 - (simulation.values[i].balance / 10);

            d.draw_line(x1, y1, x2, y2, Color::RED);
        }

        // Draw additional text
        d.draw_text(
            &format!("Simulation ID: {}", simulation.simulation_id),
            600,
            20,
            20,
            Color::BLACK,
        );
    }

}
