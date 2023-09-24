use std::env;
use glob::glob;


fn main() {
    let args: Vec<String> = env::args().collect();
    let first_dir  = args[1].clone();
    let second_dir  = args[2].clone();
    let file_paths_from_first_dir = get_file_paths_from_dir(first_dir);
    let file_paths_from_second_dir = get_file_paths_from_dir(second_dir);

    let files_in_first_and_not_second: Vec<&String> = file_paths_from_first_dir.iter().filter(|file| !file_paths_from_second_dir.contains(file)).collect();

    println!("Dateien im ersten Ordner, die nicht im zweiten sind:");
    for file in files_in_first_and_not_second{
        println!("{:?}", file);
    }

}

fn get_file_paths_from_dir(start_directory: String) -> Vec<String> {
    let all_files_in_all_dirs: String = String::from("/**/*");
    let start_directory_with_all_files = start_directory.clone() + &all_files_in_all_dirs;

    let mut file_paths_in_first_dir = vec![];
    for entry in glob(&start_directory_with_all_files).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                match path.to_str() {
                    Some(path_str) => {
                        let relative_path_str = path_str.replace(&start_directory, "");
                        file_paths_in_first_dir.push(relative_path_str);
                    }
                    None => {}
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    file_paths_in_first_dir
}



