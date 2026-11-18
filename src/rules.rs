use crate::{Map, State::{self, Alive, Dead}};

fn convert_coord_to_index(x: usize, y: usize, width: usize) -> usize{
    let index: usize = y * width + x;
    index
}
fn check_neighbour(x: usize, y: usize, map: &Map) -> u8{
    let index_of_neighbour: usize = convert_coord_to_index(x, y, map.width);
    if map.cells[index_of_neighbour] == Dead{
        return 0;
    }
    1
}
fn count_alive_neighbours(index_of_cell: usize, map: &Map) -> u8 {
    let mut nr_of_neighbours: u8 = 0;

    let x_of_cell: usize = index_of_cell % map.width;
    let y_of_cell: usize = index_of_cell / map.width;

    for dy in -1_i8..=1_i8{
        for dx in -1_i8..=1_i8{
            if !(dx == 0 && dy == 0){
                let x_neighbour_isize: isize = (x_of_cell as isize) + (dx as isize);
                let y_neighbour_isize: isize = (y_of_cell as isize) + (dy as isize);

                if x_neighbour_isize >= 0 && y_neighbour_isize >= 0{
                    let x_neighbour: usize = x_neighbour_isize as usize;
                    let y_neighbour: usize = y_neighbour_isize as usize;

                    if x_neighbour < map.height && y_neighbour < map.height{
                        nr_of_neighbours += check_neighbour(x_neighbour, y_neighbour, map);
                    }
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
    match (map.cells[index], nr_of_neighbours){
        (Alive, 0..=1) => {
            return Dead;
        }
        (Alive, 2..=3) => {
            return Alive;
        }
        (Alive, 4..) => {
            return Dead;
        }
        (Dead, 3) => {
            return Alive;
        }
        _ => {
            return map.cells[index];
        }
    }
}
//TODO - finish this
pub fn run_generation(old_map: &Map, new_map: &mut Map){
    for cell_iter in 0..old_map.cells.len() {
        new_map.cells[cell_iter] = enforce_rules(old_map, cell_iter,
            count_alive_neighbours(cell_iter, old_map));
    }
}