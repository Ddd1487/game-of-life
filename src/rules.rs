use crate::{Map, State::{self, Alive, Dead}, helper::calculate_index};

//Rules of the game
/*
1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
*/
fn cell_state_change(index: usize, new_map: &mut Map, toggle: State){
    if toggle == Dead {
        new_map.cells[index] = Dead;
    }
    else{
        new_map.cells[index] = Alive;
    }
}
// TODO - finish this
fn check_neighbours(index_of_cell: usize, map: &Map) -> u8 {
    let mut nr_of_neighbours: u8 = 0;

    for dy in -1..=1{
        for dx in -1..=1{
            if !(dx == 0 && dy == 0) {
                let index_neighbour_isize: isize = (index_of_cell as isize) + dy * (map.width as isize) + dx;
                let index_neighbour: usize = index_neighbour_isize as usize;
                if map.cells[index_neighbour] == Alive {
                    nr_of_neighbours += 1;
                }
            }
        }
    }

    nr_of_neighbours
}
pub fn run_generation(old_map: &Map){
    for cell_nr in 0..old_map.cells.len() {
        check_neighbours(cell_nr, old_map);
    }
}