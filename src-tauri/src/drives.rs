use async_recursion::async_recursion;
use tauri::async_runtime::block_on;
use winapi::um::fileapi::GetLogicalDrives;
use std::path::Path;
/*
    * Get a list of logical drives
    * @return: A vector of drive letters
*/
pub fn get_logical_drives() -> Vec<String> {
    let mut drive_list = Vec::<String>::new();
    let drives = unsafe { GetLogicalDrives() };
    for i in 0..26 {
        if drives & (1 << i) != 0 {
            let drive = (b'A' + i as u8) as char;
            drive_list.push(format!("{}:\\",drive));
        }
    }
    return drive_list;
}

/*
    * Search for files in a directory
    * @param path: The path to search
    * @param pattern: The pattern to search for  (e.g. *.txt or *.jpg or *.* or alice*.txt) 
    * @return: A vector of file paths
*/
#[tauri::command]
pub fn search(path: &str, pattern: &str) -> (Vec<String>, Vec<String>) {
    println!("Searching for files in {} with pattern {}", path, pattern);
    let mut file_list = Vec::<String>::new();
    let mut folder_list = Vec::<String>::new();
    let path = Path::new(path);
    let pattern = pattern.replace("*", ".*");
    let file_format = pattern.split(".").last().unwrap();
    let pattern = format!("{}{}", path.display(), pattern);
    println!("Searching for files in {} with pattern {}", path.display(), pattern);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let path = path.display().to_string();
            if path.contains(&pattern) && !path.contains("System Volume Information") && !path.contains("$RECYCLE.BIN") && path.ends_with(&file_format){
                if entry.path().metadata().unwrap().is_file() {
                    file_list.push(path);
                }
                else if entry.path().metadata().unwrap().is_dir() {
                    folder_list.push(path);
                }
            }
        }
    }
    println!("{:?}, {:?}", file_list, folder_list);
    return (file_list,folder_list);
}

/*
    * Search for files in a directory
    * @param path: The path to search
    * @param pattern: The pattern to search for  (e.g. *.txt or *.jpg or *.* or alice*.txt) 
    * @return: A vector of file paths
*/
#[async_recursion]
pub async fn recursive_search(path:&str, pattern: &str) -> (Vec<String>, Vec<String>){
    let mut file_list = Vec::<String>::new();
    let mut folder_list = Vec::<String>::new();
    let path = Path::new(path);
    if let Err(_e) = path.read_dir() {
        return (file_list, folder_list);
    }
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let mut path = path.display().to_string();
            while path.ends_with("\\"){
                path = path.trim_end_matches("\\").clone().to_string();
            }
            let last_part = path.split("\\").last().unwrap();
            if last_part.starts_with(&pattern) && !path.contains("System Volume Information") && !path.contains("$RECYCLE.BIN"){
                if entry.path().metadata().unwrap().is_file() {
                    file_list.push(path.clone());
                }
                else if entry.path().metadata().unwrap().is_dir() {
                    folder_list.push(path.clone());
                }
            }
            let (mut file_list2, mut folder_list2) = recursive_search(&path, pattern).await;
            file_list.append(&mut file_list2);
            folder_list.append(&mut folder_list2);
        }
    }
    return (file_list,folder_list);
}

/*
    * Search for files in a directory
    * @param path: The path to search
    * @param pattern: The pattern to search for  (e.g. *.txt or *.jpg or *.* or alice*.txt) 
    * @return: A vector of file paths
*/
#[tauri::command]
pub fn recursive_search_entrypoint(path:&str, pattern: &str) -> (Vec<String>, Vec<String>){
    let result = block_on(recursive_search(path, pattern));
    return result;
}
