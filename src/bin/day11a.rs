use advent_of_code_2023::paths::*;
#[cfg(test)]
use itertools::Itertools;

#[test]
fn test_day11a() {
    /*

    In the above example, three columns and two rows contain no galaxies:

       v  v  v
     ...#......
     .......#..
     #.........
    >..........<
     ......#...
     .#........
     .........#
    >..........<
     .......#..
     #...#.....
       ^  ^  ^
    */

    let data = include_str!("../../inputs/test_day11.txt");

    assert!(num_columns(data) == 10);
    assert!(num_rows(data) == 10);

    assert!(empty_columns(data) == vec![2, 5, 8]);
    assert!(empty_rows(data) == vec![3, 7]);

    let expanded_expected = r#"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."#
        .to_string()
        .trim()
        .to_string();
    println!("expanded:\n{}", expand_galaxies(data, None));
    println!("expected:\n{}", expanded_expected);
    assert!(expand_galaxies(data, None) == expanded_expected);

    let coords = get_coordinates(&expand_galaxies(data, None));
    assert_eq!(coords.iter().combinations(2).count(), 36);
    // dbg!(&coords);
    for wanted in [(0, 11), (5, 11), (4, 0), (0, 2), (12, 7), (9, 10)] {
        assert!(coords.contains(&wanted));
    }

    let shortest_path_list = shortest_paths(&coords);
    let sum_total: usize = shortest_path_list.iter().sum();
    dbg!(&sum_total);
    assert!(sum_total == 374)
}

#[test]
fn day11a_test_shortest_path() {
    let test_one_seven = vec![(4, 0), (9, 10)];
    let res_test_one_seven = 15;
    dbg!(shortest_paths(&test_one_seven));
    assert!(shortest_paths(&test_one_seven).contains(&res_test_one_seven));

    let test_three_six = vec![(0, 2), (12, 7)];
    let res_test_three_six = 17;
    dbg!(shortest_paths(&test_three_six));
    println!(
        "need {:?} to contain {}",
        shortest_paths(&test_three_six),
        res_test_three_six
    );
    assert!(shortest_paths(&test_three_six).contains(&res_test_three_six));

    let test_eight_nine = vec![(0, 11), (5, 11)];
    let res_test_eight_nine = 5;
    assert!(shortest_paths(&test_eight_nine).contains(&res_test_eight_nine));
}

fn main() {
    let data = include_str!("../../inputs/day11a.txt");
    println!("{}", data);
    let galaxy = expand_galaxies(data, None);
    let coords = get_coordinates(&galaxy);
    let shortest_path_list = shortest_paths(&coords);
    let sum_total: usize = shortest_path_list.iter().sum();
    println!("sum total: {}", sum_total);
}
