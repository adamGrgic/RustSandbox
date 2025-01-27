// idea: create a calculator to determine the best times to upgrade economy in zerg wars
// for zerg, we start with 50 minerals and 40 gas every 18 seconds
// each increment is 2 and can be done a maximum of 50 times. So, the end result is 150 minerals
// and 40 gas per 18 seconds.
//
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

fn main() -> std::io::Result<()> {
    // Initialize Raylib
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Simulation Visualization")
        .build();



    let num_simulations = 1000; // Number of simulations
    let total_ticks = 1500; // Simulate 600 seconds (10 minutes)

    // starting conditions
    let maximum_upgrades = 50;
    let income_interval = 18;
    let initial_balance = 500;

    let simulation_id = Uuid::new_v4();


    let results: Vec<SimulationResult> = (0..num_simulations)
        .into_iter()
        .map(|id| {

            let income_upgrade_amount = 2;

            let mut rng = rand::thread_rng();


            let mut income = 50;
            let mut income_upgrade_counts = maximum_upgrades.clone();
            let mut income_upgrade_cost = 50;
            let mut balance = initial_balance.clone();

            let mut simulation_frames: Vec<SimulationFrame> = Vec::with_capacity(total_ticks);

            for tick in 0..total_ticks {
                let x = tick as i32; // Treat tick as x
                let random_number = rng.gen_range(0..10);

                if x % income_interval == 0 {
                    balance += income;
                }


                let secret_number = 3;

                if random_number == secret_number {
                    // simulating player's RNG on getting money
                    let random_balance_addition = rng.gen_range(32..400);

                    balance += random_balance_addition;
                }


                if balance > income_upgrade_cost && income_upgrade_counts > 0 {
                    income_upgrade_counts -= 1;
                    income_upgrade_cost += income_upgrade_amount;
                    income += income_upgrade_amount;
                    balance -= income_upgrade_cost;
                }


                let simulation_frame = SimulationFrame {
                    income,
                    balance,
                    income_upgrade_cost,
                    income_upgrade_counts,
                    tick : x
                };

                simulation_frames.push(simulation_frame);
            }

            SimulationResult {
                simulation_id: id,
                values: simulation_frames,
            }
        })
        .collect();

    // Serialize results to JSON
    let json = serde_json::to_string_pretty(&results).unwrap();

    let simulation_results_file = &format!( "results_{}.json",simulation_id);



    // Write to a file
    let mut file = File::create(simulation_results_file)?;
    file.write_all(json.as_bytes())?;

    println!("Simulation complete! Results written to 'results.json'.");


     // Read simulation data from JSON file
    let file = File::open(simulation_results_file)?;
    let reader = BufReader::new(file);
    let simulations: Vec<SimulationResult> = serde_json::from_reader(reader)?;


    let simulation = &simulations[0]; // Visualize the first simulation

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


    Ok(())
}

