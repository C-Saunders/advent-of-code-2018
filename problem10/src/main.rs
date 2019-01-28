use std::fs;
use crate::parser::PointSet;

mod parser;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let mut pts = PointSet::new(&input);

    let mut current_area = pts.bounding_box().area();

    loop { 
        pts.move_points();
        let new_area = pts.bounding_box().area();

        if new_area < current_area {
            current_area = new_area;
        } else {
            println!("Min area found: {}", new_area);
            dbg!(pts.bounding_box());
            pts.print_set();

            for _ in 0..5 {
                pts.move_points_backwards();
                pts.print_set();
            }

            break;
        }
    }

    Ok(())
}
