mod exact_cover;

use exact_cover::exact_cover;

fn main() {
    println!("Hello, world!");

    let elements = vec!["a", "b", "c", "d", "e", "f", "g"];
    let sets = vec![
        vec![false, false, true, false, true, true, false],
        vec![true, false, false, true, false, false, true],
        vec![false, true, true, false, false, true, false],
        vec![true, false, false, true, false, false, false],
        vec![false, true, false, false, false, false, true],
        vec![false, false, false, true, true, false, true]
    ]; 

    let cover_set = exact_cover(elements, sets);
    for cover in cover_set {
        let mut format = String::new();
        for set in cover {
            for element in set {
                format.push_str(element);
                format.push(' ');
            }
            format.push('\n');
        }
        println!("{}", &format);
    }
}

