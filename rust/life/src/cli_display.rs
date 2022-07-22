use std::process::Command;

pub fn render(frame: Vec<Vec<u8>>, refresh: Option<bool>) {
    let image = frame
        .iter()
        .map(|vec| vec
            .iter()
            .map(|&state| if state == 1u8 {"██"} else {"  "})
            .collect::<Vec<&str>>()
            .join(""))
        .collect::<Vec<String>>()
        .join("\n");
    
    if refresh.unwrap_or_default() {
        Command::new("clear");
    }

    print!("{}", image);
}