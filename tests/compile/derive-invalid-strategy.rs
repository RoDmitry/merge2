use merge2::Merge;

#[derive(Merge)]
struct S {
    #[merge(strategy = my_custom_merge_strategy)]
    field1: u8,
}

fn my_custom_merge_strategy(left: u8, right: u8) -> u8 {
    left + right
}

fn main() {}
