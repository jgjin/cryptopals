use std::{
    collections::{
        BTreeMap,
    },
};

lazy_static! {
    pub static ref FREQUENCIES: BTreeMap<char, f32> = {
        let mut map = BTreeMap::new();
        map.insert('a', 8.167);
        map.insert('b', 1.492);
        map.insert('c', 2.782);
        map.insert('d', 4.253);
        map.insert('e', 12.702);
        map.insert('f', 2.228);
        map.insert('g', 2.015);
        map.insert('h', 6.094);
        map.insert('i', 6.966);
        map.insert('j', 0.153);
        map.insert('k', 0.772);
        map.insert('l', 4.025);
        map.insert('m', 2.406);
        map.insert('n', 6.749);
        map.insert('o', 7.507);
        map.insert('p', 1.929);
        map.insert('q', 0.095);
        map.insert('r', 5.987);
        map.insert('s', 6.327);
        map.insert('t', 9.056);
        map.insert('u', 2.758);
        map.insert('v', 0.978);
        map.insert('w', 2.360);
        map.insert('x', 0.150);
        map.insert('y', 1.974);
        map.insert('z', 0.074);

        map
    };
}
