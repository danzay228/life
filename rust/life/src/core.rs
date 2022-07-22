pub fn new_cell(state: i8, resources: i32) -> i8 {
    match (state, resources) {
        (1, 2) => 1,
        (_, 3) => 1,
        _ => 0,
    }
}

pub fn new_frame(old_frame: Vec<Vec<i8>>, resource_distribution: Vec<Vec<i32>>) -> Vec<Vec<i8>>{
    old_frame.iter().enumerate()
        .map(|(y, row)| row.iter().enumerate()
            .map(|(x, &state)|
                new_cell(state, resource_distribution[y][x]))
            .collect())
        .collect()
}

pub fn calculate_resources(frame: Vec<Vec<i8>>) -> Vec<Vec<i32>> {
    let mut resources_buffer:Vec<Vec<i32>> = frame
        .iter()
        .map(|row| vec![0i32; row.len()])
        .collect();

    for (y, row) in frame.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, state)| **state == 1) {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dx != 0 && dy != 0 {
                        let sx = match x as i32 + dx {
                            -1 => row.len()-1,
                            etc if etc == row.len() as i32 => 0,
                            etc => etc as usize,
                        };
                        let sy = match y as i32 + dy {
                            -1 => frame.len()-1,
                            etc if etc == frame.len() as i32 => 0,
                            etc => etc as usize,
                        };
                        resources_buffer[sy][sx] += 1;
                    }
                }
            }
        }
    }
    resources_buffer
}


// pub fn frame_generator(init_frame: Vec<Vec<u8>>, count: i32, buffered_frames_size: i32) -> {

// }