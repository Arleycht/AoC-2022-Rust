use std::fs;

pub fn read_input(day_number: i32, use_example: bool) -> String {
    let folder_name = if use_example { "example_inputs" } else { "real_inputs" };
    let file_path = format!("{}/day{}.txt", folder_name, day_number);
    
    fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Could not read file at path {}", file_path))
}
