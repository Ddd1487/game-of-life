use crate::Map;
//Small calculation functions
pub fn calculate_index(x: usize, y: usize, map: &Map) -> usize{
    let index: usize = y * map.width + x;
    index
}