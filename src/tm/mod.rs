use std::collections::HashMap;

pub fn start() {
    println!("Gödlenummern");

    let input = "(q1,1) = (q3,1,R)
        (q3,0) = (q1,0,R)
        (q1,s) = (q2,X,R)
        (q3,s) = (q2,S,R)"
        .to_string();

    let result = encode_input(input);

    println!("Aufgabe 1: {}", result);

    let input = "(q1,0) = (q1,0,R)
        (q1,1) = (q1,1,R)
        (q1,s) = (q3,s,R)
        (q3,1) = (q3,1,L)
        (q3,s) = (q2,1,L)
        (q3,0) = (q2,1,L)"
        .to_string();

    let result = encode_input(input);

    println!("Aufgabe 2: {}", result);
}

fn initialize_hashmap() -> HashMap<char, String> {
    let mut map = HashMap::new();

    map.insert('L', "10".to_string());
    map.insert('R', "100".to_string());

    map.insert('0', "10".to_string());
    map.insert('1', "100".to_string());
    map.insert('s', "1000".to_string());

    return map;
}

fn encode_input(input: String) -> String {
    let mut result = String::new();
    let mut symbol_codierung = initialize_hashmap();
    let lines: Vec<&str> = input.split("\n").collect();

    for line in lines {
        let line = line.trim();
        result.push_str(encode_line(line.to_string(), &mut symbol_codierung).as_str());
        result.push_str("11");
    }
    result = result.chars().take(result.chars().count() - 2).collect();

    return result;
}

fn encode_line(mut line: String, symbol_codierung: &mut HashMap<char, String>) -> String {
    let mut result = String::new();

    line = line.replace("(", "");
    line = line.replace(")", "");

    let parts: Vec<&str> = line.split("=").collect();
    let first_elements: Vec<&str> = parts[0].trim().split(",").collect();
    let second_elements: Vec<&str> = parts[1].trim().split(",").collect();

    for element in first_elements {
        result.push_str(fill_result(element, symbol_codierung).as_str());
    }
    result.push_str("1");
    for element in second_elements {
        result.push_str(fill_result(element, symbol_codierung).as_str());
    }

    return result;
}

fn fill_result(element: &str, symbol_codierung: &mut HashMap<char, String>) -> String {
    let mut result = String::new();
    // dbg!(&element);

    if element.len() == 1 {
        let ele = element.chars().nth(0).unwrap();

        if let Some(res) = symbol_codierung.get(&ele) {
            result.push_str(res);
        } else {
            let mut new_val = "1".to_string();
            for _ in 0..(3 + symbol_codierung.len() - 4) {
                new_val.push('0');
            }
            symbol_codierung.insert(ele, new_val.clone());
            result.push_str(new_val.as_str());
        }
    } else {
        let ele = element.chars().nth(1).unwrap();
        for _ in 0..ele.to_digit(10).unwrap() {
            result.push('0');
        }
    }

    return result;
}
