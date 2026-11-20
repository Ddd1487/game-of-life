use crate::{
    Map,
    State::{self, Alive, Dead},
};

pub fn toggle_state(map: &mut Map, index: usize) {
    match map.cells[index] {
        Dead => map.cells[index] = Alive,
        Alive => map.cells[index] = Dead,
    }
}
pub fn convert_coord_to_index(x: isize, y: isize, width: usize) -> usize {
    let index: usize = (y as usize) * width + (x as usize);
    index
}
fn check_neighbour(index: usize, map: &Map) -> u8 {
    match map.cells[index] {
        Alive => 1,
        Dead => 0,
    }
}
fn count_alive_neighbours(index_of_cell: usize, map: &Map) -> u8 {
    let mut nr_of_neighbours: u8 = 0;

    let x_of_cell: isize = (index_of_cell % map.width) as isize;
    let y_of_cell: isize = (index_of_cell / map.width) as isize;

    for dy in -1_isize..=1_isize {
        for dx in -1_isize..=1_isize {
            if !(dx == 0 && dy == 0) {
                let x_neighbour: isize = x_of_cell + dx;
                let y_neighbour: isize = y_of_cell + dy;
                if x_neighbour >= 0
                    && x_neighbour < (map.width as isize)
                    && y_neighbour >= 0
                    && y_neighbour < (map.height as isize)
                {
                    nr_of_neighbours += check_neighbour(
                        convert_coord_to_index(x_neighbour, y_neighbour, map.width),
                        map,
                    );
                }
            }
        }
    }
    nr_of_neighbours
}
//Rules of the game
/*
1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
*/
fn enforce_rules(map: &Map, index: usize, nr_of_neighbours: u8) -> State {
    match (map.cells[index], nr_of_neighbours) {
        (Alive, 0..=1) => Dead,
        (Alive, 2..=3) => Alive,
        (Alive, 4..) => Dead,
        (Dead, 3) => Alive,
        _ => map.cells[index],
    }
}
pub fn run_generation(old_map: &Map, new_map: &mut Map) {
    for cell_iter in 0..old_map.cells.len() {
        new_map.cells[cell_iter] = enforce_rules(
            old_map,
            cell_iter,
            count_alive_neighbours(cell_iter, old_map),
        );
    }
}
