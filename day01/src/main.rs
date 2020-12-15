// Advent of Code 2020 day 1
// Copyright (C) 2020  Victor "VoxWave" Bankowski

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{collections::HashSet, error::Error, io::prelude::*};
use std::io::BufReader;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>>  {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let mut nmbr_set = HashSet::new();
    loop {
        reader.read_line(&mut line)?;
        let num: u64 = line.trim().parse()?;
        if num < 2020 {
            if nmbr_set.contains(&(2020-num)) {
                println!("Answer: {}", num * (2020-num));
                break;
            } else {
                nmbr_set.insert(num);
            }
        }
        line.clear();
    }
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>>  {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let mut vec_o_vecs: Vec<Vec<u64>> = Vec::new();
    'outer: loop {
        reader.read_line(&mut line)?;
        let num: u64 = line.trim().parse()?;
        let mut new_vecs = Vec::new();
        for vec in vec_o_vecs.iter() {
            let cur_sum: u64 = vec.iter().sum::<u64>() + num;
            if vec.len() == 2 && cur_sum == 2020 {
                println!("{}", vec.iter().product::<u64>() * num);
                break 'outer;
            } else if cur_sum < 2020 {
                let mut clone = vec.clone();
                clone.push(num);
                new_vecs.push(clone);
            }
        }
        while let Some(vec) = new_vecs.pop() {
            vec_o_vecs.push(vec);
        }
        vec_o_vecs.push(vec![num]);

        line.clear();
    }
    Ok(())
}