use libdlx::*;

fn main() {
    println!("Hello, world!");

    let empty: Vec<Vec<&str>> = vec![];
    let res = dlx(&empty);

    // let elements = vec!["a", "b", "c", "d", "e", "f", "g"];
    // let sets = vec![
    //     vec![false, false, true, false, true, true, false],
    //     vec![true, false, false, true, false, false, true],
    //     vec![false, true, true, false, false, true, false],
    //     vec![true, false, false, true, false, false, false],
    //     vec![false, true, false, false, false, false, true],
    //     vec![false, false, false, true, true, false, true]
    // ]; 

    // let cover_set = exact_cover(&sets);
    // println!("{}", cover_set.len());
    // for cover in cover_set {
    //     let mut format = String::new();
    //     for set in cover {
    //         for (i, &val) in set.iter().enumerate() {
    //             if val {
    //                 format.push_str(elements[i]);
    //                 format.push(' ');
    //             }
    //         }
    //         format.push('\n');
    //     }
    //     println!("{}", &format);
    // }
}

