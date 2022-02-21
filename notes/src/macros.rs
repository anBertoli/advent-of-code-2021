#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

macro_rules! map {

    (nums - $( $val:tt ),*) => {{
        let mut map = std::collections::HashMap::new();
        let mut i = 0;
        $(
            map.insert(i, $val);
            i += 1;
        )*
        map
    }};

    ({ $( $key:ident : $val:expr ), * } ) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert(stringify!($key), $val); ) *
        map
    }};

    ($( $val:expr ); * ) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert(stringify!($val), $val); ) *
        map.insert(file!(), 0);
        map.insert("line", line!());
        map.insert("col", column!());
        map
    }};

     ($val:expr) => {{
        let mut map = std::collections::HashMap::new();
        map.insert(stringify!($val), $val);
        map.insert(file!(), 0);
        map.insert("line", line!());
        map.insert("col", column!());
        map
    }};
}

pub fn macro_test() {
    let hash_map = map!(1);
    println!("{:?}", hash_map);

    let hash_map = map!(1; 4; 5);
    println!("{:?}", hash_map);

    let a = 45;
    let b = 90;
    let hash_map = map!(1; a; b);
    println!("{:?}", hash_map);

    let hash_map = map!({
        pippo: 4,
        pluto: 89,
        paperino: b
    });
    println!("{:?}", hash_map);

    let hash_map = map!(nums - "Pippo", "Luca", "Antonio");
    println!("{:?}", hash_map);
}
