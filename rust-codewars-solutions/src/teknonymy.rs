use std::cmp::Reverse;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Person {
    pub name: String,
    pub sex: char,
    pub date_of_birth: chrono::NaiveDate,
    // the field you should modify, if applicable
    pub teknonym: String,
    pub children: Vec<Person>,
}

fn teknonymize(family_tree: &mut Person) {
    let _ = dfs(family_tree);
}

fn dfs(person: &mut Person) -> (u32, Reverse<chrono::NaiveDate>, &str) {
    if person.children.is_empty() {
        return (1, Reverse(person.date_of_birth), &person.name);
    }
    let (depth, dob, name) = person.children.iter_mut().map(dfs).max().unwrap();
    let by_sex = match person.sex {
        'f' => "mother",
        'm' => "father",
        _ => panic!("nuh-huh"),
    };
    person.teknonym = match depth {
        1 => format!("{by_sex} of {name}"),
        2 => format!("grand{by_sex} of {name}"),
        _ => format!(
            "{great}grand{by_sex} of {name}",
            great = "great-".repeat((depth - 2) as usize)
        ),
    };
    (depth + 1, dob, name)
}
