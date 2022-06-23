use rand::Rng;

pub const security_weight: usize = 3;
pub const special_char_weight: usize = 3;
pub const arabic_character_weight: usize = 2;

pub fn get_rand_item(subject: &Vec<String>) -> String {
    let mut range = rand::thread_rng();
    return subject[range.gen_range(0..subject.len())].clone();
}

pub fn conv_to_num(char: &String) -> usize {
    let mut result: usize = 0;
    if is_num(&char) == true {
        result = char.parse::<usize>().unwrap();
    }
    else {}
    return result;
}

pub fn get_item_index(subject: &Vec<String>, item: &String) -> usize {
    return subject.iter().position(|r| &r == &item).unwrap();
}

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: String, split_char: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}


pub fn get_char_pos(char: String) -> usize {
    let mut result: usize = 0;
    let alphabet: String = String::from(
        "abcdefghijklmnopqrstuvwxyz"
    );
    let alphabet_list: Vec<String> = clean_split(alphabet, String::from(""));
    for letter in &alphabet_list {
        if letter == &char {
            result = get_item_index(&alphabet_list.to_owned(), &letter);
        }
        else {
            // Do nothing.
        }
    }
    return result;
}

pub fn get_char_space(char_one: &String, char_two: &String) -> usize {
    let char_one_index = get_char_pos(char_one.to_string());
    let char_two_index = get_char_pos(char_two.to_string());
    let mut result: usize = 0;
    if &char_one_index > &char_two_index {
        // Do nothing.
    }
    else {
        result = char_two_index - char_one_index;
    }
    return result;
}

pub fn get_num_space(num_one: &String, num_two: &String) -> usize {
    let mut result: usize = 0;
    if conv_to_num(&num_one) > conv_to_num(&num_two) {
        // Do nothing.
    }
    else {
        result = conv_to_num(&num_two) - conv_to_num(&num_one);
    }
    return result;
}

pub fn string_type(char: &String) -> String {
    let mut result: String = String::from("int");
    if is_num(&char) == false {
        let alphabet: String = String::from(
            "abcdefghijklmnopqrstuvwxyz"
        );
        let alphabet_list: Vec<String> = clean_split(alphabet, String::from(""));
        if alphabet_list.contains(&char) == true {
            result = String::from("normChar");
        }
        else {
            result = String::from("specialChar");
        }
    }
    else {}
    return result;
} 

pub fn is_num(char: &String) -> bool {
    let mut result: bool = false;
    let match_op = char.parse::<usize>();
    match match_op {
        Ok(_x) => {
            result = true;
        },
        Err(_e) => {}
    };
    return result;
}

pub fn password_strength(password: String) -> usize {
    let mut result: usize = 0;
    let char_list: Vec<String> = clean_split(password, String::from(""));
    for item in &char_list {
        let current_item: &String = &item;
        let current_item_type: String = string_type(&item);
        let current_index: usize = get_item_index(&char_list, &item);
        if &current_index == &0 {
            // Do nothing.
        }
        else {
            let previous_item_index = current_index - 1;
        let previous_item: &String = &char_list.clone()[previous_item_index].clone();
        let previous_item_type = string_type(&previous_item);
        if current_item_type == String::from("normChar") && previous_item_type == String::from("normChar") {
            let item_space = get_char_space(&current_item, &previous_item);
            if item_space > security_weight {
                result = result + arabic_character_weight;
            } else {
                // Do nothing.
            }
        } else if current_item_type == String::from("specialChar") &&
            previous_item_type == String::from("specialChar") {
                result = result + special_char_weight;
        } else if current_item_type == String::from("int") && previous_item_type == String::from("int") {
            let itemSpace:usize = get_num_space(&current_item, &previous_item);
            if itemSpace > security_weight {
                result = result + arabic_character_weight;
            } else {
                // Do nothing.
            }
        }
        else {}
        }
    }
    return result;
}

pub fn is_secure(password: String) -> bool {
    let mut result: bool = false;
    if password_strength(password) > 8 {
        result = true;
    }
    else {}
    return result;
}

pub fn generate_password(length: usize) -> String {
    let mut result_list: Vec<String> = Vec::new();
    let alphabet_string: String = String::from(
        "abcdefghijklmnopqrstuvwxyz1234567890@_;.:"
    );
    let alphabet_list: Vec<String> = clean_split(
        alphabet_string, String::from("")
    );
    let range_end: usize = length + 1;
    for _i in 1..range_end {
        let rand_char: String = get_rand_item(&alphabet_list.clone());
        result_list.push(rand_char);
    }
    let result: String = result_list.join("");
    return result;
}

pub fn test_all(){
    println!("{:?}", get_rand_item(
        &vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d")
        ]
    ));
    println!("{:?}",get_item_index(
        &vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d")
        ],
        &String::from("b")
    ));
    println!("{:?}",conv_to_num(&String::from("2")));
    println!("{:?}",get_char_pos(String::from("d")));
    println!("{:?}",get_num_space(&String::from("v"), &String::from("d")));
    println!("{:?}",string_type(&String::from("a")));
    println!("{:?}",string_type(&String::from("1")));
    println!("{:?}",string_type(&String::from("@")));
    println!("{:?}",is_num(&String::from("a")));
    println!("{:?}",is_num(&String::from("2")));
    println!("{:?}",password_strength(String::from("1969HoglinSteak")));
    println!("{:?}",password_strength(String::from("1969HoglinSteak_@@")));
    println!("{:?}",is_secure(String::from("1969HoglinSteak")));
    println!("{:?}",is_secure(String::from("1969HoglinSteak_@@")));
    println!("{:?}",generate_password(10));


}