fn string_filter_base(strg: &String, base_len: usize) -> &str {
    if strg.len() <= base_len { //Checking input string
        return &strg[..]; //Could be "panic!("Invalid input string: {}", strg);" instead
    }

    let strg_bytes = strg.as_bytes();

    //Starting parameters for starting positions and current and max lengths:
    let mut max_len = base_len;
    let mut max_start: usize = 0;
    
    for curr_start in 0..strg_bytes.len() { //Iterating enumerated bytes of the input string
        
        let mut curr_bytes = Vec::with_capacity(base_len);
        let mut curr_len: usize = 0;
        
        'streak_iter: for i in curr_start..strg_bytes.len() {
            if curr_bytes.contains(&strg_bytes[i]) {
                curr_len += 1;
            } else if curr_bytes.len() < base_len {
                curr_bytes.push(strg_bytes[i]);
                curr_len += 1;
            } else {
                if curr_len > max_len {
                    max_len = curr_len;
                    max_start = curr_start;
                }
                break 'streak_iter;
            }
        }
    }
    &strg[max_start..(max_start + max_len)] //Returning required string slice
}

pub fn string_filter(strg: &String) -> &str {
    string_filter_base(strg, 2)
}

#[cfg(test)]
mod tests;
