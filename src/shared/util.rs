/**
 * Returns the index of the key in the vector
 */
pub fn get_index(vector: &Vec<String>, key: &str) -> i32 {
    let lookup = vector.iter().position(|v| v == key);
    match lookup {
        Some(index) => return index as i32,
        None => return -1
    }
}

/**
 * Returns the value next to the key.
 */
pub fn get_neighbor(vector: &Vec<String>, key: &str) -> Option<String> {
    // This is some crazy unwrapping
    // Basically we lookup the index of the key and return the value next to 
    // the index of the key.
    let lookup = vector.iter().position(|v| v == key);
    match lookup {
        Some(lookup) => {
            if let Some(neighbor) = vector.get(lookup + 1) {
                Some(neighbor.to_string())
            } else {
                None
            }
        },
        None => None,
    }
}