use std::path::Path;
use std::collections::HashSet;
mod engine;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
  part2();
}

const TOTAL_STEPS_P1: u32 = 1000;
const INPUT_FILE_P1: &str = "src/input_p1.txt";

#[allow(dead_code)]
fn part1() {
    let input = engine::cat(&Path::new(INPUT_FILE_P1)).unwrap();
    let mut moons = engine::parse_input(input);
    for step_counter in 0..(TOTAL_STEPS_P1 + 1) {
        if step_counter != 0 { engine::step(&mut moons); }
        println!("After {} steps:", step_counter);
        for moon in &moons {
            println!("{}", moon);
        }
    }
    println!("Total energy: {}", engine::get_total_energy(&moons));
}

const INPUT_FILE_P2: &str = "src/input_p2.txt";
const P2_MAX_STEPS: u32 = std::u32::MAX;

#[allow(dead_code)]
fn part2() {
    let input = engine::cat(&Path::new(INPUT_FILE_P2)).unwrap();
    let mut moons = engine::parse_input(input);

    let mut seen_states = HashSet::new();
    let mut step_counter = 0;
    let recv = one_second_timer();

    loop {
      // Log step if needed
      let _ = recv
      .try_recv()
      .map(|_| println!("Step {}", step_counter));

      // Run step
      if step_counter != 0 { engine::step(&mut moons); }

      // Find repeated states
      let hash = calculate_hash(&moons);
      if seen_states.contains(&hash) {
        println!("Found seen state:");
        for moon in &moons {
            println!("{}", &moon);
        }
        break;
      }
      seen_states.insert(hash);

      // Step count
      step_counter = step_counter + 1;
      if step_counter >= P2_MAX_STEPS { break; }
    }
    println!("Finished at step: {}", step_counter);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

fn one_second_timer() -> std::sync::mpsc::Receiver<&'static str> {
  let (send, recv) = channel();

  // Spawn one second timer
  thread::spawn(move || {
      loop {
          thread::sleep(Duration::from_secs(1));
          send.send("tick").unwrap();
      }
  });
  return recv;
}