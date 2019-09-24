use std::fs;

fn main() -> Result<(), String>  {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;

    Ok(())
}
