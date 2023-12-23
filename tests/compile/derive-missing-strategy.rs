use merge2::Merge;

#[derive(Merge)]
struct S {
    #[merge(strategy = my_custom_merge_strategy)]
    field1: u8,
}

fn main() {}
