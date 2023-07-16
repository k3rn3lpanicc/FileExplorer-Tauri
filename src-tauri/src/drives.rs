use async_recursion::async_recursion;
use serde::de::Error;
use tauri::async_runtime::block_on;
use winapi::um::{fileapi::{GetLogicalDrives, GetDiskFreeSpaceExW, GetVolumeInformationW, FindFirstFileW, FindNextFileW, FindClose}, winnt::{ULARGE_INTEGER, FILE_ATTRIBUTE_DIRECTORY}, minwinbase::WIN32_FIND_DATAW, handleapi::INVALID_HANDLE_VALUE};
use std::path::Path;

#[derive(Debug)]
pub struct DriveInformation {
    pub drive_letter: char,
    pub drive_type: String,
    pub drive_label: String,
    pub drive_serial_number: String,
    pub drive_file_system: String,
    pub drive_total_space: u64,
    pub drive_free_space: u64,
    pub drive_used_space: u64,
}

// impl toString for DriveInformation
impl std::fmt::Display for DriveInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"Drive Information:
    Drive Letter: {}
    Drive Type: {}
    Drive Label: {}
    Drive Serial Number: {}
    Drive File System: {}
    Drive Total Space: {}
    Drive Free Space: {}
    Drive Used Space: {}"#,
            self.drive_letter,
            self.drive_type,
            self.drive_label,
            self.drive_serial_number,
            self.drive_file_system,
            self.drive_total_space,
            self.drive_free_space,
            self.drive_used_space
        )
    }
}

pub fn get_logical_drives() -> Vec<String> {
    let mut drive_list = Vec::<String>::new();
    let drives = unsafe { GetLogicalDrives() };
    for i in 0..26 {
        if drives & (1 << i) != 0 {
            let drive = (b'A' + i as u8) as char;
            // println!("Drive {} is available", drive);
            drive_list.push(format!("{}:\\",drive));
        }
    }
    return drive_list;
}

fn _get_free_space(drive_letter: *const u16) -> (u64, u64, u64) {
    let mut free_bytes_available_to_caller: ULARGE_INTEGER = Default::default();
    let mut total_number_of_bytes: ULARGE_INTEGER = Default::default();
    let mut total_number_of_free_bytes: ULARGE_INTEGER = Default::default();
    let result = unsafe {
        GetDiskFreeSpaceExW(
            drive_letter,
            &mut free_bytes_available_to_caller,
            &mut total_number_of_bytes,
            &mut total_number_of_free_bytes,
        )
    };
    if result != 0 {
        unsafe{
            return (*total_number_of_bytes.QuadPart(), *total_number_of_free_bytes.QuadPart(), *free_bytes_available_to_caller.QuadPart());
        }
    }
    (0, 0, 0)
}


pub fn _get_logical_drive_information(drive_letter: char) -> DriveInformation {
    let mut drive_information = DriveInformation {
        drive_letter : drive_letter,
        drive_type: "".to_string(),
        drive_label: "".to_string(),
        drive_serial_number: "".to_string(),
        drive_file_system: "".to_string(),
        drive_total_space: 0,
        drive_free_space: 0,
        drive_used_space: 0,
    };
    let mut volume_name_buffer = [0u16; 256];
    let mut file_system_name_buffer = [0u16; 256];
    let mut serial_number = 0u32;
    let mut max_component_length = 0u32;
    let mut file_system_flags = 0u32;
    let mut drive_type = "".to_string();
    let mut drive_label = "".to_string();
    let mut drive_serial_number = "".to_string();
    let mut drive_file_system = "".to_string();
    let mut drive_total_space = 0u64;
    let mut drive_free_space = 0u64;
    let mut drive_available_space = 0u64;
    let drive_letter_copy = drive_letter.clone();
    let drive_letter = format!("{}:\\", drive_letter);
    let drive_letter = drive_letter.encode_utf16().collect::<Vec<u16>>();
    let drive_letter = drive_letter.as_ptr();
    let result = unsafe {
        GetVolumeInformationW(
            drive_letter,
            volume_name_buffer.as_mut_ptr(),
            volume_name_buffer.len() as u32,
            &mut serial_number,
            &mut max_component_length,
            &mut file_system_flags,
            file_system_name_buffer.as_mut_ptr(),
            file_system_name_buffer.len() as u32,
        )
    };

    if result != 0 {
        drive_type = match file_system_flags {
            0 => "Unknown".to_string(),
            1 => "Removable".to_string(),
            2 => "Fixed".to_string(),
            3 => "Remote".to_string(),
            4 => "CD-ROM".to_string(),
            5 => "RAM Disk".to_string(),
            _ => "Unknown".to_string(),
        };
        drive_label = String::from_utf16_lossy(&volume_name_buffer);
        drive_serial_number = format!("{:X}", serial_number);
        drive_file_system = String::from_utf16_lossy(&file_system_name_buffer);
        let (total_space, free_space, available_space) = _get_free_space(drive_letter);
        drive_total_space = total_space;
        drive_free_space = free_space;
        drive_available_space = available_space;
    }
    else {
        return _get_logical_drive_information(drive_letter_copy);
    }

    drive_information.drive_type = drive_type;
    drive_information.drive_label = drive_label;
    drive_information.drive_serial_number = drive_serial_number;
    drive_information.drive_file_system = drive_file_system;
    drive_information.drive_total_space = drive_total_space;
    drive_information.drive_free_space = drive_free_space;
    drive_information.drive_used_space = drive_total_space - drive_free_space;

    return drive_information;
}

pub fn get_all_drives_information() -> Vec<DriveInformation> {
    let mut drive_information_list = Vec::<DriveInformation>::new();
    let drive_list = get_logical_drives();
    for drive in drive_list {
        let drive_information = _get_logical_drive_information(drive.chars().next().unwrap());
        drive_information_list.push(drive_information);
    }
    return drive_information_list;
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
    // the B*.jpg pattern will match all jpg files starting with B
    //replace the * with a regex pattern
    let pattern = pattern.replace("*", ".*");
    let fileFormat = pattern.split(".").last().unwrap();
    let pattern = format!("{}{}", path.display(), pattern);
    println!("Searching for files in {} with pattern {}", path.display(), pattern);
    // do not use glob
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let path = path.display().to_string();
            if path.contains(&pattern) && !path.contains("System Volume Information") && !path.contains("$RECYCLE.BIN") && path.contains(&fileFormat){
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

#[async_recursion]
pub async fn recursive_search(path:&str, pattern: &str) -> (Vec<String>, Vec<String>){
    // we should search all dirs and files in the path, and their subdirs
    let mut file_list = Vec::<String>::new();
    let mut folder_list = Vec::<String>::new();
    let path = Path::new(path);
    if let Err(e) = path.read_dir() {
        // println!("Error reading dir: {}", e);
        return (file_list, folder_list);
    }
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let mut path = entry.path();
            let mut path = path.display().to_string();
            // remove all the \\ from the end of the path then get the last part
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
    // println!("{:?}, {:?}", file_list, folder_list);
    return (file_list,folder_list);
}

#[tauri::command]
pub fn recursive_search_entrypoint(path:&str, pattern: &str) -> (Vec<String>, Vec<String>){
    let result = block_on(recursive_search(path, pattern));
    return result;
}
