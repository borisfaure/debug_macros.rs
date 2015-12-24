

macro_rules! dbg {
    () => {
        dbg!("");
    };
    ($format:expr) => {
        dbg!(concat!($format,"{}"), "");
    };
    //($blk:block) => {
    //    dbg!("{:?}", $blk);
    //};
    ($format:expr, $($tail:expr),*) => {
        if cfg!(debug_assertions) {
            println!(concat!("{}:{}:{} ",$format),
                    cfg!(crate_name), file!(), line!(), $($tail),*);
        }
    };
}


#[test]
fn test_dbg() {
    dbg!();
    dbg!("");
    dbg!("non-format string");
    dbg!("format: {}", 42);
    dbg!("format: {} {}", 42, 43);
    dbg!("format: {}", 42+42);
    dbg!("format: {}", {42+42});
    //dbg!(42);
    //dbg!(42+42);
    //dbg!({ 42+42 });
}
