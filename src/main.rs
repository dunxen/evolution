// Genetic Algorithm for convergence to a given string
// Author: Duncan Dean
extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};
extern crate time;
use time::PreciseTime;

fn rand_string(size: usize) -> String {
    return thread_rng().sample_iter(&Alphanumeric).take(size).collect();
}

fn crossover(chrome1: Chromosome, chrome2: Chromosome, cross_prob: f32, solution: &str) -> Chromosome {
    if rand::thread_rng().gen_range::<f32>(0.0, 1.0) <= cross_prob {
        let code_len = chrome1.code.len();
        let rand_index = rand::thread_rng().gen_range(0, code_len);
        let first_half = &chrome1.code[0..rand_index];
        let second_half = &chrome2.code[rand_index..code_len];
        let new_code = first_half.to_string() + second_half;
        Chromosome {
            code: new_code.to_string(),
            cost_score: cost_function(&new_code, solution),
            solution: solution.to_string(),
        }
    } else if chrome1.cost_score < chrome2.cost_score {
        chrome1.clone()
    } else {
        chrome2.clone()
    }
}


fn cost_function(code: &str, solution: &str) -> u32 {
    let chars = solution.as_bytes();
    let code_chars = code.as_bytes();
    let mut cost: u32 = 0;
    for i in 0..chars.len() {
        cost += ((chars[i] as i32) - (code_chars[i] as i32)).abs() as u32;
    }
    return cost;
}


#[derive(Clone)]
struct Chromosome {
    code: String,
    cost_score: u32,
    solution: String,
}

impl Chromosome {
    fn new(solution: String) -> Chromosome {
        let random_string = rand_string(solution.len());
        Chromosome {
            code: random_string.clone(),
            cost_score: cost_function(&random_string, &solution),
            solution: solution,
        }
    }

    fn mutate(&mut self, mut_prob:f64) -> Chromosome {
        if rand::thread_rng().gen_range::<f64>(0.0, 1.0) <= mut_prob {
            let rand_index = rand::thread_rng().gen_range(0, self.code.len());
            let mut code_vec = self.code.clone().into_bytes();
            code_vec[rand_index] = rand::thread_rng().gen_range(32, 123) as u8;
            self.code = String::from_utf8(code_vec).unwrap();
            self.cost_score = cost_function(&self.code, &self.solution);
        }
        self.clone()
    }


}


fn main() {
    let solution = "Tom looks tired".to_string();
    let pop_size = 160; // Must be greater than 1
    let max_pop = pop_size * 1;
    let mut_prob = 0.3;
    let kill_constant = 0.45;
    let cross_prob = 0.5;
    let mut population = Vec::new();

    // Initialize population
    for _i in 0..pop_size {
        population.push(Chromosome::new(solution.to_string()));
        //println!("{}, cost: {}", population[i].code, population[i].cost_score);
    }


    population.sort_by(|ref a, ref b| a.cost_score.cmp(&b.cost_score));
    let mut winner: Chromosome = population[0].clone();
    // Step generation
    let mut _j = 1;

    let start = PreciseTime::now();

    while  winner.cost_score != 0 {

        // Mutations
        for i in 0..population.len() {
            population[i] = population[i].mutate(mut_prob);
        }

        // Kill the weaklings
        population.sort_by(|ref a, ref b| a.cost_score.cmp(&b.cost_score));
        for _ in 0..((( population.len() as f32)*kill_constant) as u32) {
            population.pop();
        }



        // Crossovers
        if population.len() < max_pop {
            for i in 0..(population.len() - 2) {
                let chrome1 = population[i].clone();
                let chrome2 = population[i+1].clone();
                population.push(crossover(chrome1, chrome2, cross_prob, &solution));
            }
        }

        population.sort_by(|ref a, ref b| a.cost_score.cmp(&b.cost_score));
        winner = population[0].clone();
        //println!("\nGeneration: {}\nString: {}\nCost: {}\n", j, winner.code, winner.cost_score);

        _j += 1;

    }

    let end = PreciseTime::now();
    println!("Benchmark Time: {}", start.to(end));


}
