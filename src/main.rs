mod rules;

#[derive(Clone)]
struct Map{
    width: usize,
    height: usize,
    cells: Vec<State>,
}
#[derive(Clone,PartialEq)]
enum State{
    Dead,
    Alive,
}
fn simulate(mut generation: usize, limit: usize){
    while generation < limit {
        
        generation += 1;
    }
}
fn main() {
    //println!("Hello, world!");
    let width = 10;
    let height = 10;
    let test_map_old = Map{
        width,
        height,
        cells: vec![State::Dead; width * height],
    };
    let test_map_new = test_map_old.clone();

    simulate(0, 100);
}
