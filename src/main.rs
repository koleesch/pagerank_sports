// The PageRank strcut holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: u32,
}

impl PageRank {
    fn new(damping: f64, iterations: u32) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates the PageRank of each node in the graph.
    fn rank(&self, graph: &[Vec<usize>]) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank of each node.
        let mut ranks = vec![1.0 / n as f64; n];

        // iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank of each node.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value to distribute to its linked nodes.
                let contribution = ranks[node] / edges.len() as f64;

                // Distributes the PageRank value to its linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Calculates the new PageRank of each node.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / n as f64;
            }

            // Replaces the old PageRank with the new PageRank.
            ranks = new_ranks;
        }

        ranks
    }
}

fn main() {
    // The graph represents links between sports websites. Each index represents a website,
    // and the values in the vector are indexes of the websites they link to.
    let graph = vec![
        vec![1, 2], // ESPN links to NFL and NBA.
        vec![0],    // NFL links to ESPN.
        vec![0, 3], // NBA links to ESPN and UFC.
        vec![0],    // UFC links to ESPN.
        vec![0, 1], // MLB links to ESPN and NFL.
    ];

    // Corresponding to the indexes of the websites in the graph.
    let names = ["ESPN", "NFL", "NBA", "UFC", "MLB"];

    // initializes the PageRank struct with a damping factor of 0.85 and 100 iterations.
    let pagerank = PageRank::new(0.85, 100);

    // Calculates the PageRank values
    let ranks = pagerank.rank(&graph);

    // Prints the PageRank values of each website.
    for (name, rank) in names.iter().zip(ranks) {
        println!("The Pangerank of {} is {:.4}", name, rank);
    }
}
