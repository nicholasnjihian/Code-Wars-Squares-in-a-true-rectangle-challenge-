fn get_squares(rect_len: i32, rect_width: i32) -> Option<Vec<i32>> {
    let mut area: i32 = rect_len * rect_width;
    let mut v: Vec<i32> = Vec::new();
    loop {
        let (r, d): (i32, i32) = iterate_through_vals(area);
        v.push(r);
        if d == 0_i32 {
            break;
        }
        area = d;
    }
    Some(v)
}

fn iterate_through_vals(prod: i32) -> (i32, i32) {
    let root: f32 = ((prod.clone() as f32).sqrt()).trunc();
    let diff: f32 = (prod as f32) - (root * root);
    (root as i32, diff as i32)
}
fn main() {
    let vals = match get_squares(5, 3) {
        Some(v) => v,
        None => panic!("No value was found as no value was given!"),
    };
    println!("{:?}", vals);
}
