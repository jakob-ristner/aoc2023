use std::collections::HashMap;

pub static TO_WEST: [char; 3] = ['-', 'F', 'L'];
pub static TO_EAST: [char; 3] = ['-', '7', 'J'];
pub static TO_NORTH: [char; 3] = ['|', 'F', '7'];
pub static TO_SOUTH: [char; 3] = ['|', 'J', 'L'];
pub type Pos = (i32, i32);
pub type Map = HashMap<Pos, char>;

pub fn get_next(map: &Map, &(row, col): &Pos, &(pr, pc): &Pos) -> Pos {
    let curr_ch = map.get(&(row, col)).unwrap();

    if let Some(ch) = map.get(&(row + 1, col)) {
        if (TO_SOUTH.contains(ch) || ch == &'S')
            && (pr, pc) != (row + 1, col)
            && TO_NORTH.contains(curr_ch)
        {
            return (row + 1, col);
        }
    }
    if let Some(ch) = map.get(&(row - 1, col)) {
        if (TO_NORTH.contains(ch) || ch == &'S')
            && (pr, pc) != (row - 1, col)
            && TO_SOUTH.contains(curr_ch)
        {
            return (row - 1, col);
        }
    }
    if let Some(ch) = map.get(&(row, col + 1)) {
        if (TO_EAST.contains(ch) || ch == &'S')
            && (pr, pc) != (row, col + 1)
            && TO_WEST.contains(curr_ch)
        {
            return (row, col + 1);
        }
    }
    if let Some(ch) = map.get(&(row, col - 1)) {
        if (TO_WEST.contains(ch) || ch == &'S')
            && (pr, pc) != (row, col - 1)
            && TO_EAST.contains(curr_ch)
        {
            return (row, col - 1);
        }
    }

    unreachable!();
}

pub fn get_next_initial(map: &Map, &(row, col): &Pos) -> Pos {
    if let Some(ch) = map.get(&(row + 1, col)) {
        if TO_SOUTH.contains(ch) {
            return (row + 1, col);
        }
    }
    if let Some(ch) = map.get(&(row - 1, col)) {
        if TO_NORTH.contains(ch) {
            return (row - 1, col);
        }
    }
    if let Some(ch) = map.get(&(row, col + 1)) {
        if TO_EAST.contains(ch) {
            return (row, col + 1);
        }
    }
    if let Some(ch) = map.get(&(row, col - 1)) {
        if TO_WEST.contains(ch) {
            return (row, col - 1);
        }
    }

    unreachable!();
}
pub fn part_1(path: &Vec<Pos>) -> usize {
    (path.len() - 1) / 2
}
