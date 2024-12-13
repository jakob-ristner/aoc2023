use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque}, primitive,
};

type Pos = (i64, i64);

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    pos: Pos,
    cost: i64,
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
    let (graph, _) = build_graph(content);
    let w = graph[0].len();
    let h = graph.len();
    let start = (w as i64 / 2, h as i64 / 2);
    let p1 = bfs(&graph, &start, 64);
    let p2 = part2(&graph, 26501365);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part2(graph: &[Vec<char>], max_steps: i64) -> i64 {
    let start = (graph.len() as i64 / 2, graph.len() as i64 / 2);
    let edge = graph.len() as i64 / 2;
    let y0 = edge;
    let y1 = edge + graph.len() as i64;
    let y2 = edge + 2 * graph.len() as i64;
    let fst = bfs(graph, &start, y0);
    let snd = bfs(graph, &start, y1);
    let thr = bfs(graph, &start, y2);
    let a = (thr - (2 * snd) + fst) / 2;
    let b = snd - fst - a;
    let c = fst;
    let n = (max_steps - edge) / graph.len() as i64;
    a * n * n + b * n + c
}

fn bfs(graph: &[Vec<char>], start: &Pos, max_steps: i64) -> i64 {
    let mut visited = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let stepsmod = max_steps % 2;
    let w = graph[0].len();
    let h = graph.len();
    heap.push(State {
        pos: *start,
        cost: 0,
    });
    while let Some(State { pos, cost }) = heap.pop() {
        if visited.contains_key(&pos) {
            continue;
        }
        visited.insert(pos, cost);
        for adj in adj(&pos) {
            if is_wall_wrapping(graph, &adj) {
                continue;
            }
            if visited.contains_key(&adj) {
                continue;
            }
            if cost + 1 > max_steps {
                continue;
            }
            heap.push(State {
                pos: adj,
                cost: cost + 1,
            });
        }
    }
    visited.iter().filter(|(_, v)| *v % 2 == stepsmod).count() as i64
}

fn adj(pos: &Pos) -> Vec<Pos> {
    let (x, y) = pos;
    vec![(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)]
}

fn is_wall_wrapping(graph: &[Vec<char>], pos: &Pos) -> bool {
    let (x, y) = pos;
    let w = graph[0].len() as i64;
    let h = graph.len() as i64;
    let x = *x % w;
    let y = *y % h;
    let x = if x < 0 { x + w } else { x };
    let y = if y < 0 { y + h } else { y };
    graph[y as usize][x as usize] == '#'
}

fn build_graph(input: &str) -> (Vec<Vec<char>>, Pos) {
    let charmat = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let w = charmat.len() as i64;

    let start = (w / 2, w / 2);
    (charmat, start)
}
