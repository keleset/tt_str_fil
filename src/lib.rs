use std::collections::VecDeque;

struct StreakList {
    //Represents queue of characters
    list: VecDeque<(usize, usize, char)>, //byte_pos, char_pos, char
    max_size: usize,                      //Max size of the queue
}

impl StreakList {
    //Constructor
    fn new(max_size: usize) -> StreakList {
        StreakList {
            list: VecDeque::new(),
            max_size,
        }
    }

    fn push(&mut self, new_elem: (usize, usize, char)) -> Option<(usize, usize, char)> {
        //Method for pushing values to the queue
        'list_iter: for (i, &elem) in self.list.iter().enumerate() {
            //Check if the queue already contains this char
            if elem.2 == new_elem.2 {
                //If it does, we need to move it to the beginning, so remove it from the old position:
                self.list.remove(i);
                break 'list_iter;
            }
        }

        self.list.push_back(new_elem); //Pushing tuple (position_in_text, character) to the queue

        if self.list.len() > self.max_size {
            //If queue overfilled, pop and return the oldest character
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
    let mut max_byte_len: usize = 0;
    let mut max_char_len: usize = 0;
    let mut max_byte_start: usize = 0;
    let mut streak_byte_len: usize = 0;
    let mut streak_char_len: usize = 0;
    let mut streak_byte_start: usize = 0;

    let mut streak_chars = StreakList::new(base_len);

    //Iterating through characters of the input string:
    'strg_iter: for (curr_char_pos, (curr_byte_pos, curr_char)) in strg.char_indices().enumerate() {
        match streak_chars.push((curr_byte_pos, curr_char_pos, curr_char)) {
            //Something returned, that means something was popped
            //from the queue as result of pushing through max_size limit,
            //that means that streak is broken and now we have to
            //record streak (if worth recording) and
            //use the data about the position of the oldest
            //(popped) character from the queue to determine the start
            //position for new streak:
            Some((drop_byte_pos, drop_char_pos, drop_char)) => {
                if streak_char_len > max_char_len {
                    max_char_len = streak_char_len;
                    max_byte_len = streak_byte_len;
                    max_byte_start = streak_byte_start;
                }
                streak_byte_start = drop_byte_pos + drop_char.len_utf8();
                streak_char_len = curr_char_pos - drop_char_pos;
                streak_byte_len =
                    curr_byte_pos + curr_char.len_utf8() - drop_byte_pos - drop_char.len_utf8();
            }
            //None returned means that streak continues:
            None => {
                streak_byte_len += curr_char.len_utf8();
                streak_char_len += 1;
            }
        }
    }
    //Check in case of streak is in the end of the string:
    if streak_char_len > max_char_len {
        max_byte_len = streak_byte_len;
        max_byte_start = streak_byte_start;
    }
    &strg[max_byte_start..(max_byte_start + max_byte_len)] //Returning required string slice
}

pub fn string_filter(strg: &String) -> &str {
    string_filter_base(strg, 2)
}

#[cfg(test)]
mod tests;
