use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

#[derive(Hash, Clone, Copy)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


#[derive(Hash)]
pub struct Moon {
    id: i32,
    pos: Vec3,
    vel: Vec3,
}
impl std::fmt::Display for Moon {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>", self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z)
    }
}

pub fn get_total_energy(moons: &Vec<Moon>) -> i32 {
    let mut energy = 0;
    for moon in moons {
        let potential_energy = moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs();
        let kinetic_energy = moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs();
        energy = energy + potential_energy * kinetic_energy;
        println!("pot: {} kin: {} ({})", potential_energy, kinetic_energy, moon);
    }
    return energy;
}

fn get_gravity_diff(pos1: &Vec3, pos2: &Vec3) -> Vec3 {
    let x = if pos1.x == pos2.x { 0 } else { if pos1.x < pos2.x { 1 } else { -1 } };
    let y = if pos1.y == pos2.y { 0 } else { if pos1.y < pos2.y { 1 } else { -1 } };
    let z = if pos1.z == pos2.z { 0 } else { if pos1.z < pos2.z { 1 } else { -1 } };
    
    return Vec3 {
        x, y, z
    }
}

pub fn step(moons: &mut Vec<Moon>) {
    // Step 1: Apply gravity
    for index1 in 0..moons.len() {
        for index2 in (index1 + 1)..moons.len() {
            let gravity_diff = get_gravity_diff(&moons[index1].pos, &moons[index2].pos);
            moons[index1].vel += gravity_diff;
            moons[index2].vel -= gravity_diff;
        }
    }
    // Step 2: Apply velocity
    for moon in moons {
        moon.pos += moon.vel;
    }
}

pub fn parse_input(input: String) -> Vec<Moon> {
    let lines = input.split('\n');
    let mut moons: Vec<Moon> = Vec::new();

    let x_reg = Regex::new(r"x=(-?[0-9]+)").unwrap();
    let y_reg = Regex::new(r"y=(-?[0-9]+)").unwrap();
    let z_reg = Regex::new(r"z=(-?[0-9]+)").unwrap();
    let mut id = 0;
    for line in lines {
        id = id + 1;
        let x = x_reg.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = y_reg.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let z = z_reg.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let new_moon = Moon {
            id: id,
            pos: Vec3 { x, y, z },
            vel: Vec3 { x: 0, y: 0, z: 0 }
        };
        moons.push(new_moon);
    }
    return moons;
}

pub fn cat(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}