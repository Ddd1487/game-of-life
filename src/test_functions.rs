use crate::{Map, State::{self, Alive}, rules::{convert_coord_to_index, run_generation}};
fn test_print(map: &Map){
    for iter in 0..map.size {
        if iter % map.width == 0 {
            println!();
        }
        if map.cells[iter] == Alive {
            print!("X ");
        }
        else{
            print!("  ");
        }
    }
}
fn test_manual_start(map: &mut Map){
    map.cells[convert_coord_to_index(2, 1, map.width)] = Alive;
    map.cells[convert_coord_to_index(1, 2, map.width)] = Alive;
    map.cells[convert_coord_to_index(2, 2, map.width)] = Alive;
    map.cells[convert_coord_to_index(3, 2, map.width)] = Alive;
    map.cells[convert_coord_to_index(2, 3, map.width)] = Alive;
}
pub fn test(width: usize, height: usize, limit_of_generations: usize){
    let mut old_map = Map{
        width,
        height,
        cells: vec![State::Dead; width * height],
        size: width * height,
    };
    let mut new_map = old_map.clone();

    print!("Starting position\n");
    test_manual_start(&mut old_map);
    test_print(&old_map);
    print!("\n\n\n");

    let mut generations: usize = 0;
    while generations < limit_of_generations {
        print!("Generation number: {}\n", generations + 1);
        run_generation(&old_map, &mut new_map);
        test_print(&new_map);
        std::mem::swap(&mut old_map, &mut new_map);
        generations += 1;
    }
}