use advent_of_code_2023::paths::*;

#[test]
fn test_day11b() {
    // (In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

    let data = include_str!("../../inputs/test_day11.txt");
    let galaxy = expand_galaxies(data, Some(10));
    let coords = get_coordinates(&galaxy);
    let shortest_path_list = shortest_paths(&coords);
    let sum_total: usize = shortest_path_list.iter().sum();
    assert_eq!(sum_total, 1030);

    let galaxy = expand_galaxies(data, Some(100));
    let coords = get_coordinates(&galaxy);
    let shortest_path_list = shortest_paths(&coords);
    let sum_total: usize = shortest_path_list.iter().sum();
    assert_eq!(sum_total, 8410);
}

fn main() {
    let data = include_str!("../../inputs/day11a.txt");
    // println!("{}", data);
    let galaxy = expand_galaxies(data, Some(1000000));
    let coords = get_coordinates(&galaxy);
    let shortest_path_list = shortest_paths(&coords);
    let sum_total: usize = shortest_path_list.iter().sum();
    println!("sum total: {}", sum_total);
}
