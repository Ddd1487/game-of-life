use crate::{Map, State::{self, Alive, Dead}, helper::calculate_index, neighbours::{case_bot_left, case_bot_line, case_bot_right, case_left_line, case_right_line, case_top_line}};

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

    if index_of_cell < map.width{
        if index_of_cell % map.width == 0{
            nr_of_neighbours += case_top_left(index_of_cell,map);
        }
        else if index_of_cell % map.width == map.width - 1{
            nr_of_neighbours += case_top_right(index_of_cell,map);
        }
        else{
            nr_of_neighbours += case_top_line(index_of_cell,map);
        }
    }
    else if index_of_cell >= map.size - map.width{
        if index_of_cell % map.width == 0{
            nr_of_neighbours += case_bot_left(index_of_cell, map);
        }
        else if index_of_cell & map.width == map.width - 1{
            nr_of_neighbours += case_bot_right(index_of_cell, map);
        }
        else{
            nr_of_neighbours += case_bot_line(index_of_cell, map);
        }
    }
    else if index_of_cell % map.width == 0{
        nr_of_neighbours += case_left_line(index_of_cell, map);
    }
    else if index_of_cell % map.width == map.width - 1{
        nr_of_neighbours += case_right_line(index_of_cell, map);
    }
    else{
        nr_of_neighbours += case_mid(index_of_cell, map);
    }

    nr_of_neighbours
}
//TODO - finish this
pub fn run_generation(old_map: &Map){
    for cell_nr in 0..old_map.cells.len() {
        let nr_of_neighbours: u8 = check_neighbours(cell_nr, old_map);
    }
}