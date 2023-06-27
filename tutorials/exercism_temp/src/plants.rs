use std::collections::HashMap;

fn main() {
    // let res = is_armstrong_number(3999999999);
    // println!("{}", res);
    let a = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";
    let _t = "ABCDEFGHIJKLMNOPQRSTUVYWXYZABCDEFGHIJKLMNOPQRSTUV";
    let b = "Bob";
    let res = plants(a, b);
    println!("{:?}", res);
}

// pub fn is_armstrong_number(num: u32) -> bool {
//     let mut result: u32 = 0;
//     let num_string = format!("{}", num);
//     let length = num_string.len();
//     for value in num_string.chars() {
//         let added_value = value.to_digit(10).unwrap().pow(length as u32);
//         if let Some(new_result) = result.checked_add(added_value) {
//             result = new_result;
//         } else {
//             return false;
//         }
//     }
//     result == num
// }
//     println!("{} : {}", _diagram, _student);
pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let mut result = Vec::new();
    let mut types = HashMap::new();
    types.insert('V', "violets");
    types.insert('R', "radishes");
    types.insert('G', "grass");
    types.insert('C', "clover");

    let student_first_character = _student.as_bytes().first().unwrap();
    let student_position = ((student_first_character % 65) * 2) as usize;
    println!("student position : {}", student_position);
    let mut trimmed_diagram = _diagram.split('\n');
    let first_row = trimmed_diagram.next().unwrap();
    let first_result = &first_row[student_position..student_position + 2];
    println!("{}", first_result);
    let second_row = trimmed_diagram.next().unwrap();
    let second_result = &second_row[student_position..student_position + 2];
    println!("{}", second_result);

    let plants = first_result.to_owned() + second_result;
    for plant in plants.chars() {
        result.push(types[&plant]);
    }
    result
}

#[test]
fn test_garden_with_single_student() {
    let diagram = "RC
GG";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_different_garden_with_single_student() {
    let diagram = "VC
RC";
    let student = "Alice";
    let expected = vec!["violets", "clover", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_garden_with_two_students() {
    let diagram = "VVCG
VVRC";
    let student = "Bob";
    let expected = vec!["clover", "grass", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_second_students_garden() {
    let diagram = "VVCCGG
VVCCGG";
    let student = "Bob";
    let expected = vec!["clover", "clover", "clover", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_third_students_garden() {
    let diagram = "VVCCGG
VVCCGG";
    let student = "Charlie";
    let expected = vec!["grass", "grass", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_alice_first_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Alice";
    let expected = vec!["violets", "radishes", "violets", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_bob_second_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Bob";
    let expected = vec!["clover", "grass", "clover", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_charlie() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Charlie";
    let expected = vec!["violets", "violets", "clover", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_david() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "David";
    let expected = vec!["radishes", "violets", "clover", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_eve() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Eve";
    let expected = vec!["clover", "grass", "radishes", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_fred() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Fred";
    let expected = vec!["grass", "clover", "violets", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_ginny() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Ginny";
    let expected = vec!["clover", "grass", "grass", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_harriet() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Harriet";
    let expected = vec!["violets", "radishes", "radishes", "violets"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_ileana() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Ileana";
    let expected = vec!["grass", "clover", "violets", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_joseph() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Joseph";
    let expected = vec!["violets", "clover", "violets", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_kincaid_second_to_last_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Kincaid";
    let expected = vec!["grass", "clover", "clover", "grass"];
    assert_eq!(plants(diagram, student), expected);
}
#[test]

fn test_for_larry_last_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Larry";
    let expected = vec!["grass", "violets", "clover", "violets"];
    assert_eq!(plants(diagram, student), expected);
}

// const  CHILDRENS:[&str;12] = ["Alice", "Bob", "Charlie","David","Eve", "Fred", "Ginny", "Harriet","Ileana", "Joseph", "Kincaid", "Larry"];
// pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {

//     let position = CHILDRENS.iter().position(|&n| n==_student).unwrap() * 2;

//     let apply_char = |c:char| match c {
//         'V' => "violets",
//         'R' => "radishes",
//         'C' => "clover",
//         'G' => "grass",
//         _   => ""
//     };
//     _diagram.lines().flat_map(|line|
//         {
//         line[position..=position+1].chars().map(apply_char)
//     })
//     .collect()
// }
