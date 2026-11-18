use crate::{Map, State::{self, Alive}, rules::run_generation};
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
fn test_simulate(limit_of_generations: usize, old_map: &Map, new_map: &mut Map){
    let generations: usize = 0;
    while generations < limit_of_generations {
        print!("Generation number: {}\n", generations + 1);
        run_generation(old_map,new_map);
        test_print(new_map);
    }
}
pub fn test(width: usize, height: usize, limit_of_generations: usize){
    let old_map = Map{
        width,
        height,
        cells: vec![State::Dead; width * height],
        size: width * height,
    };
    let mut new_map = old_map.clone();

    test_simulate(limit_of_generations, &old_map, &mut new_map);
}