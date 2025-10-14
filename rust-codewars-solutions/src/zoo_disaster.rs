use std::collections::HashMap;
pub fn who_eats_who(zoo: &str) -> Vec<String> {
    let diet = HashMap::from([
        ("big-fish", vec!["little-fish"]),
        ("bug", vec!["leaves"]),
        ("chicken", vec!["bug"]),
        ("cow", vec!["grass"]),
        ("giraffe", vec!["leaves"]),
        ("panda", vec!["leaves"]),
        ("sheep", vec!["grass"]),
        ("antelope", vec!["grass"]),
        (
            "bear",
            vec!["big-fish", "bug", "chicken", "cow", "leaves", "sheep"],
        ),
        ("fox", vec!["chicken", "sheep"]),
        ("lion", vec!["antelope", "cow"]),
    ]);

    let mut stack = Vec::new();
    let mut res = vec![zoo.to_string()];

    for animal in zoo.split(",").into_iter() {
        while !stack.is_empty() {
            let prev_animal = stack.last().unwrap();
            if diet.get(animal).unwrap_or(&vec![]).contains(prev_animal) {
                res.push(format!("{animal} eats {prev_animal}"));
                stack.pop();
            } else {
                break;
            }
        }

        if !stack.is_empty() {
            let prev_animal = stack.last().unwrap();
            if diet.get(prev_animal).unwrap_or(&vec![]).contains(&animal) {
                res.push(format!("{prev_animal} eats {animal}"))
            } else {
                stack.push(animal);
            }
        } else {
            stack.push(animal);
        }
    }

    res.push(stack.join(","));
    res
}
