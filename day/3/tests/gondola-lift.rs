use gondola_lift;
use std::fs::read_to_string;

// fn get_engine_schematic() -> Vec<&'static str> {
//     let content = read_to_string("./example-engine-schematic.txt").expect("Invalid input file");
//     let lines: Vec<&str> = content.lines().collect();
//
//     lines
// }

// #[test]
// fn is_part_number_467() {
//     let input = get_engine_schematic();
//     let
//     assert!(false);
// }

// #[test]
// fn is_part_number_114() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_35() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_633() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_617() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_58() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_592() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_755() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_664() {
//     assert!(false);
// }
//
// #[test]
// fn is_part_number_598() {
//     assert!(false);
// }

#[test]
fn get_part_numbers_sum() {
    let input = read_to_string("./example-engine-schematic.txt").expect("Invalid input file");
    let lines: Vec<&str> = input.lines().collect();
    let part_numbers: Vec<u32> = gondola_lift::get_part_numbers(&lines);
    assert_eq!(part_numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    assert_eq!(part_numbers.iter().sum::<u32>(), 4361);
}
