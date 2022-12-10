use std::{collections::HashSet, io::Read};

struct CastingData {
    num_roles: u16,
    num_scenes: u16,
    num_actors: u16,
    role_data: Vec<HashSet<u16>>,
    scene_data: Vec<HashSet<u16>>,
}

fn main() {
    let data = parse_input();
}

fn parse_input() -> CastingData {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer.split_whitespace().map(|x| x.parse::<u16>().unwrap());

    let mut next = || input.next().unwrap();

    let num_roles = next();
    let num_scenes = next();
    let num_actors = next();

    let mut roles: Vec<HashSet<u16>> = Vec::with_capacity(num_roles as usize);
    for _ in 0..num_roles {
        let n = next();
        let mut possible_actors: HashSet<u16> = HashSet::with_capacity(n as usize);
        for _ in 0..n {
            possible_actors.insert(next());
        }
        roles.push(possible_actors);
    }

    let mut scenes: Vec<HashSet<u16>> = Vec::with_capacity(num_scenes as usize);
    for _ in 0..num_scenes {
        let n = next();
        let mut roles_included = HashSet::with_capacity(n as usize);
        for _ in 0..n {
            roles_included.insert(next());
        }
        scenes.push(roles_included);
    }

    // Return parsed data
    CastingData {
        num_roles,
        num_scenes,
        num_actors,
        role_data: roles,
        scene_data: scenes,
    }
}
