use crate::rules::run_generation;

mod rules;

#[derive(Clone)]
struct Map{
    width: usize,
    height: usize,
    cells: Vec<State>,
    size: usize,
}
#[derive(Clone,PartialEq)]
enum State{
    Dead,
    Alive,
}
fn simulate(mut generation: usize, limit: usize, old_map: &Map){
    while generation < limit {
        run_generation(old_map);
        generation += 1;
    }
}
fn main() {
    //Testing
    let width = 10;
    let height = 10;
    let test_map_old = Map{
        width,
        height,
        cells: vec![State::Dead; width * height],
        size: width * height,
    };
    let _test_map_new = test_map_old.clone();

    simulate(0, 100, &test_map_old);
}
