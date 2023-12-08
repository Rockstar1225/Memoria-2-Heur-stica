//! A* algorithm trait implementation

use std::{
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

use super::{Cost, State};

/// Search statistics struct
#[derive(Default)]
pub struct Stats {
    /// Total time required
    pub time: Duration,
    /// Amount of nodes expanded before finding the optimal path
    pub expanded: usize,
}

/// Search result struct
pub struct SearchResult<T: State> {
    /// Optimal path found
    pub path: Vec<T>,
    /// Cost of the path found
    pub cost: usize,
}

/// Generic interface to run an A* search on a given graph
pub trait AStar {
    type State: State + std::fmt::Debug;
    /// Expands a node, generating its successors
    ///
    /// # Parameters
    ///
    /// * `state`: Node to expand
    fn next(&self, state: &Self::State) -> Vec<(Self::State, usize)>;

    /// Determines if a node is a goal
    ///
    /// # Parameters
    ///
    /// * `state`: Node to check
    fn is_goal(&self, state: &Self::State) -> bool;

    /// Performs an A* search
    ///
    /// # Parameters
    ///
    /// * `start`: Starting node
    /// * `h`: Heuristic function calculating the estimated cost to reach a goal
    fn a_star(
        &self,
        start: Self::State,
        h: impl Fn(&Self::State) -> usize,
    ) -> (Option<SearchResult<Self::State>>, Stats) {
        let start_time = Instant::now();
        let mut stats = Stats::default();
        // Set of nodes to expand
        let mut open = BinaryHeap::new();
        open.push(Cost {
            state: start.clone(),
            cost: 0,
        });
        // Best known cost to node
        let mut g_score = HashMap::new();
        g_score.insert(start, (0, None));
        while let Some(Cost { state, cost }) = open.pop() {
            // Displays a progress indication
            if stats.expanded % 1_000_000 == 0 {
                let expanded = stats.expanded / 1_000_000;
                println!("Current expanded: {expanded}M\tCurrent estimated cost: {cost}");
            }
            stats.expanded += 1;
            // If the state is a goal, reconstructs the path and returns the stats
            if self.is_goal(&state) {
                stats.time = start_time.elapsed();
                println!("Optimal path found. Reconstructing path...");
                return (
                    Some(SearchResult {
                        path: Self::reconstruct_path(g_score, state, h),
                        cost,
                    }),
                    stats,
                );
            }
            // Expands the node
            for (neighbor, cost) in self.next(&state) {
                // Calculates the cost of a path to `neighbor` going through `state`
                let tentative_g_score = g_score[&state].0 + cost;
                let entry = g_score.get(&neighbor);
                // If there was no known path to neighbor/the known path was worse,
                // updates the best known path to neighbor and adds it to open set again
                if entry.map_or(true, |x| tentative_g_score < x.0) {
                    g_score.insert(neighbor.clone(), (tentative_g_score, Some(state.clone())));
                    open.push(Cost {
                        cost: tentative_g_score + h(&neighbor),
                        state: neighbor,
                    });
                }
            }
        }
        stats.time = start_time.elapsed();
        (None, stats)
    }

    /// Reconstructs the best known path to the given node
    ///
    /// # Parameters
    ///
    /// * `g_score`: Map of nodes to their parent
    /// * `end`: Node for which to reconstruct the path
    fn reconstruct_path(
        g_score: HashMap<Self::State, (usize, Option<Self::State>)>,
        end: Self::State,
        h: impl Fn(&Self::State) -> usize,
    ) -> Vec<Self::State> {
        // Initializes the result vector with the end node
        let mut out = vec![end.clone()];
        let mut current = end;
        // While the current node has a parent, add that parent to the result vector
        while let (cost, Some(prev)) = &g_score[&current] {
            let heu = h(&current);
            println!("Cost: {cost},\tHeuristic: {heu},\tTotal: {}", cost + heu);
            // println!("{current:?}");
            out.push(prev.clone());
            current = prev.clone();
        }
        // Reverses the result vector so it starts on the start node
        out.reverse();
        out
    }
}
