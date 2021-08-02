use std::collections::{HashSet, HashMap};

fn get_set_with_str(names: &[&str]) -> HashSet<String> {
    let mut set = HashSet::new();

    for name in names.iter() {
        set.insert(name.to_string());
    }
    set
}

fn get_set_with_strslice<'a>(names: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for name in names.iter() {
        set.insert(*name);
    }
    set
}

fn get_strslice_from_strslice(name: &str) -> &str {
    name
}

// rustc: cannot return value referencing function parameter `name` returns a value referencing data owned by the current function
/*
fn get_strslice_from_str<'b>(name: String) -> &'b str {
    &name[..]
}
*/

fn main() {
    let mut set = HashSet::new();
    let mut map = HashMap::new();

    set.insert("david");
    map.insert("david", "tang");

    dbg!(set);
    dbg!(map);


    let new_set = get_set_with_str(&[ "david", "tang" ]);
    dbg!(new_set);

    let new_set2 = get_set_with_strslice(&[ "david", "tang" ]);
    dbg!(new_set2);

    let new_str = get_strslice_from_strslice("david");
    dbg!(new_str);

    /*
    let new_str2 = get_strslice_from_str("david".to_string());
    dbg!(new_str2);
    */
}
