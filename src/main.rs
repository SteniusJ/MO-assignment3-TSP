// Sligthly over commented file to explain Rust syntax if it is unfamiliar

use itertools::Itertools;
use rand::random_range;

// Array matrix, an array of N size containing a array of N size containing usigned 64 bit integers
type Distances = [[u64; N]; N];

// Traits are like Java interfaces
trait TSP {
    fn bruteforce_alg(&self) -> (u64, Vec<usize>);
    fn random_alg(&self, initial_temp: f64) -> (u64, Vec<usize>);
}

// Trait implementations for Distance type
impl TSP for Distances {
    fn bruteforce_alg(&self) -> (u64, Vec<usize>) {
        //! Bruteforce TSP solving algorithm
        let mut min_cost = u64::MAX;
        let mut optimal_route: Vec<_> = Vec::new();

        /*
        1..self.len() <- rust range, same in python => range(1, len(self))
        .permutations - gives me all the possible combinations of the input data in n sized slices
        .for_each - basic "for x in y" loop
        */
        (1..self.len())
            .permutations(self.len() - 1)
            .for_each(|perm| {
                let mut curr_cost = 0;
                let mut curr_node = 0;

                for i in 0..perm.len() {
                    curr_cost += self[curr_node][perm[i]];
                    curr_node = perm[i];
                }

                curr_cost += self[curr_node][0];

                if min_cost > curr_cost {
                    // add 0 to beginning of optimal route
                    // since it is the starting point
                    optimal_route = [vec![0], perm].concat();
                }

                min_cost = min_cost.min(curr_cost);
            });

        //return
        (min_cost, optimal_route)
    }

    fn random_alg(&self, initial_temp: f64) -> (u64, Vec<usize>) {
        /*!
        Simulated Annealing
        Some sort of version of simulated annealing
        Not the most optimized due to multiple vector mutations and clones
        */
        const COOLING_FACTOR: f64 = 0.1;

        let mut temp = initial_temp;
        // Create Vector (Array list in Java terms) of all possible permutations
        // of the range 1..self.len()
        let mut solutions = (1..self.len())
            .permutations(self.len() - 1)
            .collect::<Vec<Vec<usize>>>();

        let mut best_solution: (u64, Vec<usize>) = (u64::MAX, vec![]);

        while temp > 0.0 {
            let perm = random_range(..solutions.len());
            let solution = &solutions[perm];
            // Calculate cost of current selected sollution
            // and assign that value to "cost" variable
            let cost = {
                let mut cost = 0;
                let mut node = 0;

                for i in 0..solution.len() {
                    cost += self[node][solution[i]];
                    node = solution[i];
                }

                // return
                cost + self[node][0]
            };

            // If solution is the best one yet
            // overwrite "best_solution" with the current one
            if best_solution.0 > cost {
                // WARNING Hotloop clone here fix this
                best_solution = (cost, solution.clone());
            }

            // Remove solution from solutions
            solutions.remove(perm);

            if solutions.is_empty() {
                break;
            }

            // Cool temperature
            temp -= COOLING_FACTOR;
        }

        // Insert 0 at the beginning of the "best_solution" path vector
        // this is due to every path staring from point 0 since starting
        // position doesn't matter in circular TSP solutions
        best_solution.1.insert(0, 0);
        // return
        best_solution
    }
}

const N: usize = 5;

const DISTANCES: Distances = [
    [0, 12, 10, 19, 8],
    [12, 0, 3, 7, 6],
    [10, 3, 0, 6, 20],
    [19, 7, 6, 0, 4],
    [8, 6, 20, 4, 0],
];

fn main() {
    println!("{:?}", DISTANCES.random_alg(0.3));
    println!("{:?}", DISTANCES.bruteforce_alg());
}
