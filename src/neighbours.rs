use crate::{Map, State::{Alive, Dead}};

fn is_it_alive(index: usize, map: &Map) -> bool{
    if map.cells[index] == Dead{
        false;
    }
    true
}

pub fn case_top_left(index: usize, map: &Map) -> u8{
    let mut nr_of_neighbours: u8 = 0;
    if mid_right(index, map){
        nr_of_neighbours += 1;
    }
    if bot_right(index, map){
        nr_of_neighbours += 1;
    }
    if bot_mid(index, map){
        nr_of_neighbours += 1;
    }
    nr_of_neighbours
}
pub fn case_top_line(index: usize, map: &Map) -> u8{
    let mut nr_of_neighbours: u8 = 0;
    if mid_left(index, map){
        nr_of_neighbours += 1;
    }
    if mid_right(index, map){
        nr_of_neighbours += 1;
    }
    if bot_left(index, map){
        nr_of_neighbours += 1;
    }
    if bot_mid(index, map){
        nr_of_neighbours += 1;
    }
    if bot_right(index, map){
        nr_of_neighbours += 1;
    }
    nr_of_neighbours
}
pub fn case_top_right(index: usize, map: &Map) -> u8{
    let mut nr_of_neighbours: u8 = 0;
    if mid_left(index, map){
        nr_of_neighbours += 1;
    }
    if bot_left(index, map){
        nr_of_neighbours += 1;
    }
    if bot_mid(index, map){
        nr_of_neighbours += 1;
    }
    nr_of_neighbours
}
pub fn case_left_line(index: usize, map: &Map){

}
pub fn case_right_line(index: usize, map: &Map){

}
pub fn case_bot_line(index: usize, map: &Map){

}
pub fn case_bot_left(index: usize, map: &Map){

}
pub fn case_bot_right(index: usize, map: &Map){

}
pub fn case_mid(index: usize, map: &Map){

}

fn top_left(index: usize, map: &Map) -> bool {
    let index_new: usize = index - map.width - 1;
    is_it_alive(index_new, map)
}
fn top_mid(index: usize, map: &Map) -> bool {
    let index_new: usize = index - map.width;
    is_it_alive(index_new, map)
}
fn top_right(index: usize, map: &Map) -> bool {
    let index_new: usize = index - map.width + 1;
    is_it_alive(index_new, map)
}
fn mid_left(index: usize, map: &Map) -> bool {
    let index_new: usize = index - 1;
    is_it_alive(index_new, map)
}
fn mid_right(index: usize, map: &Map) -> bool {
    let index_new: usize = index + 1;
    is_it_alive(index_new, map)
}
fn bot_left(index: usize, map: &Map) -> bool {
    let index_new: usize = index + map.width - 1;
    is_it_alive(index_new, map)
}
fn bot_mid(index: usize, map: &Map) -> bool {
    let index_new: usize = index + map.width;
    is_it_alive(index_new, map)
}
fn bot_right(index: usize, map: &Map) -> bool {
    let index_new: usize = index + map.width + 1;
    is_it_alive(index_new, map)
}