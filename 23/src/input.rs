pub fn get_input_for_current_exe() -> String {
    let exe = std::env::current_exe().unwrap();
    let project_dir = exe
        .ancestors()
        .filter(|p| p.is_dir())
        .find(|d| d.file_name().unwrap() == "aoc23")
        .unwrap()
        .to_path_buf();
    let input_file = project_dir.join("input").join(exe.file_name().unwrap());
    std::fs::read_to_string(input_file).unwrap()
}
