use std::collections::HashMap;

use icss_utils::replace_value_symbols;

fn main() {
    let mut map = HashMap::default();
    map.insert("red", "blue");
    println!("{}", replace_value_symbols("0 0 0 4px red", map));

}

