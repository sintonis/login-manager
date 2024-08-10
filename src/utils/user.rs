use std::fs::File;
use std::io::{self, BufRead};
use pwd::Passwd;
use crate::constants::{DEFAULT_UID_MAX, DEFAULT_UID_MIN, LOGIN_DEFINITION_FILE};

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub dir: String,
    pub shell: String
}


impl User {

    /// Extract and return the UID_MIN and UID_MAX values of the Linux system.
    fn get_uid_min_max() -> io::Result<(u32, u32)> {
        let file = File::open(LOGIN_DEFINITION_FILE)?;
        let reader = io::BufReader::new(file);

        let mut uid_min = None;
        let mut uid_max = None;

        for line in reader.lines() {
            let line = line.unwrap();

            if line.starts_with("UID_MIN") {
                if let Some(value) = line.split_whitespace().nth(1).and_then(|v| v.parse::<u32>().ok()) {
                    uid_min = Some(value);
                }
            } else if line.starts_with("UID_MAX") {
                if let Some(value) = line.split_whitespace().nth(1).and_then(|v| v.parse::<u32>().ok()) {
                    uid_max = Some(value);
                }
            }

            // Break the loop once both values are found
            if uid_min.is_some() && uid_max.is_some() {
                break;
            }
        }

        // Use default values if not found
        let uid_min = uid_min.unwrap_or(DEFAULT_UID_MIN);
        let uid_max = uid_max.unwrap_or(DEFAULT_UID_MAX);

        Ok((uid_min, uid_max))
    }

    fn get_full_name_from_gecos(gecos: &str) -> Option<String> {
        gecos.split(',').next().map(|name| name.to_string()).filter(|name| !name.is_empty())
    }

    /// Retrieves all non-system users within the specified UID range.
    pub fn get_all() -> Vec<User> {
        let (uid_min, uid_max) = Self::get_uid_min_max().unwrap();
        
        let mut users: Vec<User> = Vec::new();

        // Iterate through all users in the system
        for passwd in Passwd::iter() {

            // Skip users outside the specified UID range
            if passwd.uid < uid_min || passwd.uid > uid_max {
                continue;
            }

            // Extract the full name from the GECOS field, or fallback to the username if GECOS is not available
            let name = passwd.gecos
                .as_deref()
                .and_then(|gecos| Self::get_full_name_from_gecos(gecos))
                .unwrap_or_else(|| passwd.name.clone());

            users.push(User {
                name,
                uid: passwd.uid,
                gid: passwd.gid,
                dir: passwd.dir.clone(),
                shell: passwd.shell.clone(),
            });
        }
        
        users
    }

    fn process_list(users: &Vec<User>) {
        for user in users {
            println!("Processing user with id: {}", user.name);
        }
    }
}