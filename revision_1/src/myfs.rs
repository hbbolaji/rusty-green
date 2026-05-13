use std::{fs, path::Path};

pub fn test_create_dir() {
    let path_str = "./data";
    let my_path = Path::new(path_str);
    if my_path.exists() {
        println!("Directory already exist...");
        return;
    }

    let create_dir_result = fs::create_dir(path_str);

    if create_dir_result.is_ok() {
        println!("created new data directory...")
    } else {
        println!(
            "Some problem occurred while creating dir {:?}",
            create_dir_result.err().unwrap().kind()
        );
    }
}

pub fn test_create_files() {
    let path_str_1 = "./data/file_1.txt";
    let path_str_2 = "./data/file_2.txt";
    let path_str_3 = "./data/file_3.txt";

    let contents_1 = "Hashim Bello";
    let contents_2 = "Ibrahim Bello";
    let contents_3 = "Zuzu Bello";

    fs::write(path_str_1, contents_1);
    fs::write(path_str_2, contents_2);
    fs::write(path_str_3, contents_3);

    // fs::remove_file(path_str_2);
}

pub fn test_remove_dir() {
    let path_str = "./data";
    fs::remove_dir_all(path_str);
}

pub fn test_read_somefile() {
    let file_path = "./data/file_1.txt";
    let read_result = fs::read(file_path);
    println!(
        "{:?}",
        read_result
            .unwrap()
            .iter()
            .fold(String::new(), |mut acc: String, v| {
                acc.push(char::from(*v));
                return acc;
            })
    );
}
