use crate::{Map, State::{self, Alive, Dead}};

//Rules of the game
/*
1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
*/
pub fn cell_state_change(index: usize, new_map: &mut Map, die_or_born: State){
    if die_or_born == Dead {
        new_map.cells[index] = Dead;
    }
    else{
        new_map.cells[index] = Alive;
    }
}
