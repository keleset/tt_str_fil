use std::collections::VecDeque;

struct StreakList { //Represents queue of characters
    list: VecDeque<(usize, char)>,
    max_size: usize, //Max size of the queue
}

impl StreakList { //Constructor
    fn new(max_size: usize) -> StreakList {
        StreakList {
            list: VecDeque::new(),
            max_size
        }
    }

    fn push(&mut self, new_elem: (usize, char)) -> Option<(usize, char)>{ //Method for pushing values to the queue
        'list_iter: for (i, &elem) in self.list.iter().enumerate() { //Check if the queue already contains this char
            if elem.1 == new_elem.1 { //If it does, we need to move it to the beginning, so remove it from the old position:
                self.list.remove(i);
                break 'list_iter;
            }
        }

        self.list.push_back(new_elem); //Pushing tuple (position_in_text, character) to the queue
        
        if self.list.len() > self.max_size { //If queue overfilled, pop and return the oldest character
            return self.list.pop_front();
        }
        None //Returning None if queue is still in limits
    }
}

fn string_filter_base(strg: &String, base_len: usize) -> &str {
    
    //Checking input string:
    if strg.len() <= base_len {
        return &strg[..];
    }

    //Storing length and starting point for max and current streak:
    let mut max_len = base_len;
    let mut max_start: usize = 0;
    let mut curr_len: usize = 0;
    let mut curr_start: usize = 0;

    let mut streak_bytes = StreakList::new(base_len);

    //Iterating through characters of the input string:
    'strg_iter: for (i, curr_char) in strg.char_indices() {

        match streak_bytes.push((i, curr_char)) {
            //Something returned, that means something was popped
            //from the queue as result of pushing through max_size limit,
            //that means that streak is broken and now we have to
            //record streak (if worth recording) and
            //use the data about the position of the oldest
            //(popped) character from the queue to determine the start
            //position for new streak:
            Some((drop_elem_pos, _drop_elem)) => {
                if curr_len > max_len {
                    max_len = curr_len;
                    max_start = curr_start;
                }
                curr_start = drop_elem_pos + curr_char.len_utf8();
                curr_len = i - drop_elem_pos;
            }
            //None returned means that streak continues:
            None => {
                curr_len += curr_char.len_utf8();
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
