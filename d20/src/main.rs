use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Map = Vec<Vec<char>>;
type Pos = (i32, i32);

// Up, right, down, left
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, Eq, PartialEq)]
struct State {
    path: Vec<Pos>,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let content = include_str!("input.txt");
    let map = map(content);
    let start = start(&map);
    let p1 = num_cheats(&map, &start, 2, 100);
    let p2 = num_cheats(&map, &start, 20, 100);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn num_cheats(map: &Map, start: &Pos, ch_len: u32, min_saved: i32) -> usize {
    let path = bfs(&map, &start).unwrap();
    let p_len = path.len();
    let cost_to_end: HashMap<Pos, i32> = path
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, (p_len - i - 1) as i32))
        .collect();
    let s_cost = cost_to_end[&start];
    let mut cheats: HashMap<(Pos, Pos), i32> = HashMap::new();
    for (cost_to_here, &pos) in path.iter().enumerate() {
        for (opp, offset) in reachable_in_n_steps(&map, &pos, ch_len) {
            let cost_from_opp = cost_to_end[&opp];
            let cost = cost_to_here as i32 + offset + cost_from_opp;
            let saved = s_cost - cost;
            if saved < min_saved {
                continue;
            }
            cheats.insert((pos, opp), cost);
        }
    }
    cheats.len()
}

fn reachable_in_n_steps(map: &Map, from: &Pos, n: u32) -> Vec<(Pos, i32)> {
    let mut out = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let x = x as i32;
            let y = y as i32;
            if (x, y) == *from {
                continue;
            }
            let dx = x.abs_diff(from.0);
            let dy = y.abs_diff(from.1);
            if dx + dy > n {
                continue;
            }
            if map[y as usize][x as usize] == '#' {
                continue;
            }
            out.push(((x, y), (dx + dy) as i32));
        }
    }
    out
}
fn map(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn start(map: &Map) -> Pos {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start position found");
}

fn bfs(map: &Map, start: &Pos) -> Option<Vec<Pos>> {
    let w = map[0].len();
    let h = map.len();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<Pos, i32> = HashMap::new();
    assert_ne!(map[start.1 as usize][start.0 as usize], '#');
    heap.push(State {
        path: vec![*start],
        cost: 0,
    });

    while let Some(State { path, cost }) = heap.pop() {
        let pos = path.last().unwrap();
        if visited.contains_key(pos) && visited[pos] < cost {
            continue;
        }
        visited.insert(*pos, cost);
        let (x, y) = *pos;
        if map[y as usize][x as usize] == 'E' {
            return Some(path);
        }

        for new_pos in adj_bounded(pos, w, h) {
            if map[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                let mut new_path = path.clone();
                new_path.push(new_pos);
                heap.push(State {
                    path: new_path,
                    cost: cost + 1,
                });
            }
        }
    }
    None
}

fn adj_bounded(pos: &Pos, w: usize, h: usize) -> Vec<Pos> {
    let mut adj = Vec::new();
    for &(dx, dy) in DIRS.iter() {
        let new_pos = (pos.0 + dx, pos.1 + dy);
        if new_pos.0 >= 0 && new_pos.0 < w as i32 && new_pos.1 >= 0 && new_pos.1 < h as i32 {
            adj.push(new_pos);
        }
    }
    adj
}
