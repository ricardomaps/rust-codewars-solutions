use std::collections::HashSet;
// n is the number of lights
// corresponding_lights_list is the array representing relationships between lights and switches
// returns a boolean, represents whether it is possible to turn all the lights on.
pub fn light_switch(n: u8, corresponding_lights_list: &Vec<Vec<u8>>) -> bool {
    let end_state = (1 << n) - 1;
    let switches = corresponding_lights_list
        .iter()
        .map(|ls| ls.iter().fold(0, |acc, l| acc | (1 << l)))
        .collect::<Vec<u32>>();
    let mut states = HashSet::from([0]);
    for switch in switches {
        let new_states: Vec<u32> = states.iter().map(|s| s ^ switch).collect();
        states.extend(new_states);
        if states.contains(&end_state) {
            return true;
        }
    }
    false
}
