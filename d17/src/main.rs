use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Pos = (u32, u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rev(&self) -> Self {
        use Dir::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

type Graph = HashMap<Pos, Vec<(u32, Pos, Dir)>>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    pos: Pos,
    dir: Dir,
    cost: u32,
    step: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let content = include_str!("input.txt");
    let (graph, start, goal) = build_graph(content);
    let c1 = bfs(&graph, start, goal, 0, 3);
    let c2 = bfs(&graph, start, goal, 4, 10);

    println!("Part 1: {}", c1);
    println!("Part 2: {}", c2);
}

fn bfs(graph: &Graph, start: Pos, goal: Pos, minstep: u32, maxstep: u32) -> u32 {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    // pos dir stepnum
    let mut visited: HashSet<(Pos, Dir, u32)> = HashSet::new();

    for adj in &graph[&start] {
        heap.push(State {
            pos: adj.1,
            dir: adj.2,
            cost: adj.0,
            step: 1,
        });
    }

    while let Some(State {
        pos,
        dir,
        cost,
        step,
    }) = heap.pop()
    {
        if pos == goal {
            return cost;
        }
        if visited.contains(&(pos, dir, step)) {
            continue;
        }
        visited.insert((pos, dir, step));

        for (adj_cost, adj_pos, adj_dir) in &graph[&pos] {
            let new_step = if *adj_dir == dir { step + 1 } else { 1 };
            if new_step > maxstep
                || *adj_dir == dir.rev()
                || (step < minstep && *adj_dir != dir)
            {
                continue;
            }
            heap.push(State {
                pos: *adj_pos,
                dir: *adj_dir,
                cost: cost + adj_cost,
                step: new_step,
            });
        }
    }
    unreachable!("No path found");
}

fn build_graph(input: &str) -> (Graph, Pos, Pos) {
    let mut graph = Graph::new();

    let w = input.lines().next().unwrap().len() as u32;
    let h = input.lines().count() as u32;
    let charmat = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            let adj = adj(&pos, w as usize, h as usize)
                .into_iter()
                .map(|((x, y), dir)| {
                    let c = charmat[y as usize][x as usize].to_digit(10).unwrap();
                    (c, (x, y), dir)
                })
                .collect::<Vec<_>>();
            graph.insert(pos, adj);
        }
    }

    let start = (0, 0);
    let goal = (w - 1, h - 1);
    (graph, start, goal)
}

fn adj(pos: &Pos, w: usize, h: usize) -> Vec<(Pos, Dir)> {
    let mut res = Vec::new();
    if pos.0 > 0 {
        res.push(((pos.0 - 1, pos.1), Dir::Left));
    }
    if pos.0 < w as u32 - 1 {
        res.push(((pos.0 + 1, pos.1), Dir::Right));
    }
    if pos.1 > 0 {
        res.push(((pos.0, pos.1 - 1), Dir::Up));
    }
    if pos.1 < h as u32 - 1 {
        res.push(((pos.0, pos.1 + 1), Dir::Down));
    }
    res
}
