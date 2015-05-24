extern crate rand;

use rand::{Rng, thread_rng};
use rand::distributions::Range;
use individual::Individual;

mod individual;

fn select<'a, R: Rng>(population: &'a Vec<Individual>, rng: &mut R)
                      -> &'a Individual {
    let population: Vec<_> = (0..4).map(|_| rng.choose(population)).collect();
    if let Some(selected) = population.iter().min() {
        return selected.unwrap();
    }
    unimplemented!();
}

fn main() {
    let mut rng = thread_rng();
    let range = Range::new(-512.03_f64, 511.97); // range for Schwefel problem

    // initialize population
    let mut population: Vec<_> = (0..128).map(|_| {
        Individual::new(&range, &mut rng)
    }).collect();

    for i in 0..10000 {
        // select and mutate individuals for next population
        population = (0..128).map(|_| {
            select(&population, &mut rng).mutate(&range, &mut rng)
        }).collect();

        let best = population.iter().min().unwrap();
        if i % 100 == 0 {
            println!("{}th fitness: {}", i, best.fitness);
        }

        if best.fitness < 1000_f64 {
            println!("{}th solution converged at {}: {:?}",
                     i, best.fitness, best.solution);
            return;
        }
    }
    println!("Failed to converge.");
}
