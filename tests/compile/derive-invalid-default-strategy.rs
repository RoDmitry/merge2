use merge2::Merge;

#[derive(Merge)]
#[merge(strategy = my_custom_merge_strategy)]
struct S {
    field1: u16,
}

fn my_custom_merge_strategy(left: &mut u8, right: u8) {
    *left += right
}

fn main() {}

