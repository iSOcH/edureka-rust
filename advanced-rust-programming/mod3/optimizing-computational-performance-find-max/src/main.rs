use std::{error::Error, time::SystemTime};

use optimizing_computational_performance_find_max::FindMax;

pub mod single_threaded;
pub mod multi_threaded;

fn main() -> Result<(), Box<dyn Error>> {
    let num_runs = 10;
    let num_elements: usize = 4_000_000_000;
    println!("Initializing data for {num_elements} items");

    let data = &mut vec![0u32; num_elements][..];

    for (idx, val) in data.iter_mut().enumerate() {
        *val = idx.try_into().unwrap();
    }

    println!("Data initialized");

    let subjects: Vec<Box<dyn FindMax>> = vec![
        Box::new(single_threaded::SingleThreadedNaive),
        Box::new(single_threaded::StdLib),
        Box::new(single_threaded::SimdArgmaxCrate),
        Box::new(multi_threaded::Threaded::new(2, Box::new(single_threaded::SimdArgmaxCrate))),
        Box::new(multi_threaded::Threaded::new(4, Box::new(single_threaded::SimdArgmaxCrate))),
        Box::new(multi_threaded::Threaded::new(8, Box::new(single_threaded::SimdArgmaxCrate))),
        Box::new(multi_threaded::Threaded::new(16, Box::new(single_threaded::SimdArgmaxCrate))),
    ];

    for run_nr in 1..=num_runs {
        for sut in &subjects {
            let start = SystemTime::now();
            print!("Run Nr {run_nr}: Starting with {sut:?}... ");
            let result = sut.find_max(data);
    
            println!("Run Nr {run_nr}: Found {result:?} after {:?}", start.elapsed());
        }

        println!();
    }

    Ok(())
}