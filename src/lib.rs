fn string_filter_base(strg: &String, base_len: usize) -> &str {
    
    //Checking input string:
    if strg.len() <= base_len {
        return &strg[..];
    }
    
    //Dividing strg to byte array
    let strg_bytes = strg.as_bytes();

    //Storing max length and starting point for max length streak:
    let mut max_len = base_len;
    let mut max_start: usize = 0;

    //Iterating through bytes of the input string:
    for curr_start in 0..strg_bytes.len() { 
        
        let mut curr_bytes = Vec::with_capacity(base_len); //Vector used as storage for characters of the current streak
        let mut curr_len: usize = 0; //Length of the current streak
        
        //Checking each byte as beginning of the streak:
        'streak_iter: for i in curr_start..strg_bytes.len() {

            if curr_bytes.contains(&strg_bytes[i]) { //Checking if storage already contains current character
                curr_len += 1; //Yes, increase current streak length
            } else if curr_bytes.len() < base_len { //No, but storage is not full yet
                curr_bytes.push(strg_bytes[i]); //Adding character to storage
                curr_len += 1; //Increasing streak
            } else { //New character, but storage is full:
                if curr_len > max_len { //Store the result, if worth storing
                    max_len = curr_len;
                    max_start = curr_start;
                }
                break 'streak_iter; //Breaking streak
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
