use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn count_provinces() -> String {
    let level_list = generate_level_list("district.json");
    let mut ret: Vec<String> = vec![];
    for mut level in level_list {
        make_symmetric(&mut level);
        let num = get_area_count(level);
        ret.push(num.to_string());
    }
    ret.join(",")
}

fn generate_level_list(path: &str) -> Vec<HashMap<String, Vec<String>>> {
    let path = Path::new(path);
    let content = fs::read_to_string(path).unwrap();
    get_deserialization_data(content)
}

fn get_deserialization_data(content: String) -> Vec<HashMap<String, Vec<String>>> {
    let string = content.replace("\n", "").replace(" ", "").replace("\"", "");

    let mut data: Vec<Vec<(String, Vec<String>)>> = Vec::new();

    let set_level_2_data =
        |data: &mut Vec<Vec<(String, Vec<String>)>>, point: usize, index: usize| {
            if let Some(vec) = data.last_mut() {
                if let Some(map) = vec.last_mut() {
                    let word = string
                        .chars()
                        .skip(point + 1)
                        .take(index - point - 1)
                        .collect();
                    map.1.push(word);
                }
            }
        };

    let mut json_level_stack = vec![];
    let mut point = 0;
    for (index, char) in string.chars().enumerate() {
        if char.to_string().eq("{") || char.to_string().eq("[") {
            json_level_stack.push((char, index));
            point = index;
            continue;
        }
        if char.to_string().eq("]") {
            set_level_2_data(&mut data, point, index);
            json_level_stack.pop();
            point = index;
            continue;
        }
        if char.to_string().eq("}") {
            json_level_stack.pop();
            point = index;
            continue;
        }
        if char.to_string().eq(",") {
            if json_level_stack.len() == 3 {
                set_level_2_data(&mut data, point, index);
            }
            point = index;
            continue;
        }
        if char.to_string().eq(":") {
            if json_level_stack.len() == 1 {
                data.push(Vec::new());
                continue;
            }
            if json_level_stack.len() == 2 {
                if let Some(vec) = data.last_mut() {
                    let word = string
                        .chars()
                        .skip(point + 1)
                        .take(index - point - 1)
                        .collect();
                    vec.push((word, Vec::new()));
                }
            }
        }
    }

    let mut result = Vec::new();
    for d in data {
        let mut map = HashMap::new();
        for mut city_connect in d {
            map.entry(city_connect.0)
                .or_insert(Vec::new())
                .append(&mut city_connect.1);
        }
        result.push(map);
    }
    result
}

fn make_symmetric(area_map: &mut HashMap<String, Vec<String>>) {
    let mut edges = vec![];
    for (city, connect) in area_map.clone() {
        for c in connect {
            if !area_map.get(&c).unwrap_or(&Vec::new()).contains(&city) {
                edges.push((c.clone(), city.clone()));
            }
        }
    }
    for (from, to) in edges {
        area_map.entry(from).or_insert_with(Vec::new).push(to);
    }
}

fn get_area_count(area_map: HashMap<String, Vec<String>>) -> u32 {
    let mut visited = HashSet::new();
    let mut count = 0;
    for city in area_map.keys() {
        if !visited.contains(city) {
            count += 1;
            dfs(city, &area_map, &mut visited);
        }
    }
    count
}

fn dfs(city: &String, area_map: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    visited.insert(city.clone());
    if let Some(cities) = area_map.get(city) {
        for city in cities {
            if !visited.contains(city) {
                dfs(city, area_map, visited);
            }
        }
    }
}
