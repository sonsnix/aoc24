use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Coordinates,
    dir: usize,
    score: i32,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    CW,
    CCW,
    Forward,
}

const ACTIONS: [Action; 3] = [Action::CW, Action::CCW, Action::Forward];
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Maze {
    walls: HashSet<Coordinates>,
    start: Coordinates,
    end: Coordinates,
}

impl Maze {
    /// Parses the input map into a `Grid`.
    fn from_input(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = Coordinates { x: 0, y: 0 };
        let mut end = Coordinates { x: 0, y: 0 };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = Coordinates {
                            x: x as i32,
                            y: y as i32,
                        };
                    }
                    'E' => {
                        end = Coordinates {
                            x: x as i32,
                            y: y as i32,
                        };
                    }
                    '#' => {
                        walls.insert(Coordinates {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    _ => (),
                }
            }
        }

        Maze { walls, start, end }
    }

    /// Finds the shortest path from start to end.
    fn find_shortest_path(&self) -> (i32, usize) {
        let initial_state = State {
            pos: self.start,
            dir: 0,
            score: 0,
        };

        // create queue of (state, path)
        let mut queue = VecDeque::from([(initial_state, vec![initial_state.pos])]);

        // keep track of visited states
        let mut visited = HashMap::new();

        let mut best_path_positions = HashSet::new();
        let mut best_score = i32::MAX;

        while let Some((state, path)) = queue.pop_front() {
            if state.pos == self.end {
                if state.score < best_score {
                    best_score = state.score;
                    best_path_positions = path.into_iter().collect();
                } else if state.score == best_score {
                    best_path_positions.extend(path.iter());
                }
                continue;
            }

            if let Some(&prev_score) = visited.get(&(state.pos, state.dir)) {
                if prev_score < state.score {
                    continue;
                }
            }

            visited.insert((state.pos, state.dir), state.score);

            if state.score >= best_score {
                continue;
            }

            queue.extend(
                ACTIONS
                    .iter()
                    .filter_map(|&action| self.apply_action(state, action))
                    .map(|next_state| (next_state, [path.clone(), vec![next_state.pos]].concat())),
            );
        }

        (best_score, best_path_positions.len())
    }

    fn _visualize_paths(&self, path: HashSet<Coordinates>) {
        let max_x = self.walls.iter().max_by_key(|c| c.x).unwrap().x;
        let max_y = self.walls.iter().max_by_key(|c| c.y).unwrap().y;

        for y in 0..=max_y {
            for x in 0..=max_x {
                let c = Coordinates { x, y };

                if self.walls.contains(&c) {
                    print!("#");
                } else if path.contains(&c) {
                    print!("O");
                } else if c == self.start {
                    print!("S");
                } else if c == self.end {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!();
    }

    /// Applies an action to a given state, returning the resulting state if valid.
    fn apply_action(&self, state: State, action: Action) -> Option<State> {
        let mut new_pos = state.pos;
        let mut new_dir = state.dir;
        let mut new_score = state.score;

        match action {
            Action::CW => {
                new_dir = (state.dir + 1) % 4;
                new_score += 1000;
            }
            Action::CCW => {
                new_dir = (state.dir + 3) % 4;
                new_score += 1000;
            }
            Action::Forward => {
                new_pos = Coordinates {
                    x: state.pos.x + DIRS[state.dir].0,
                    y: state.pos.y + DIRS[state.dir].1,
                };
                if self.walls.contains(&new_pos) {
                    return None;
                }
                new_score += 1;
            }
        }

        Some(State {
            pos: new_pos,
            dir: new_dir,
            score: new_score,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = Maze::from_input(&input);

    let (best_score, num_best_path_positions) = grid.find_shortest_path();

    println!("Part 1: {}", best_score);
    println!("Part 2: {}", num_best_path_positions);
}
