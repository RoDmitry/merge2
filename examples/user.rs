use merge2::Merge;

#[derive(Merge)]
struct User {
    #[merge(skip)]
    pub name: &'static str,
    pub location: Option<&'static str>,
    #[merge(strategy = ::merge2::vec::append)]
    pub groups: Vec<&'static str>,
}

fn main() {
    let defaults = User {
        name: "",
        location: Some("Internet"),
        groups: vec!["rust"],
    };
    let mut ferris = User {
        name: "Ferris",
        location: None,
        groups: vec!["mascot"],
    };
    ferris.merge(defaults);

    assert_eq!("Ferris", ferris.name);
    assert_eq!(Some("Internet"), ferris.location);
    assert_eq!(vec!["mascot", "rust"], ferris.groups);
}
