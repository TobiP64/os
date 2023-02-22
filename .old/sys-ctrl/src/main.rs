use std::sync::Arc;

struct Texture {
    width:  usize,
    height: usize,
    data:   &'static [u8]
}

struct TopLevelSelection {
    icon:  Option<Texture>,
    label: &'static str,
    sub:   &'static [SubSelection]
}

struct SubSelection {
    icon:   Option<Texture>,
    label:  &'static str
}

static SELECTION: [TopLevelSelection; 3] = [
    TopLevelSelection { icon: None, label: "System Monitor", sub: &[
        SubSelection { icon: None, label: "Processes" },
        SubSelection { icon: None, label: "Resources" },
        SubSelection { icon: None, label: "Sockets" }
    ] },
    TopLevelSelection { icon: None, label: "System Control", sub: &[] },
    TopLevelSelection { icon: None, label: "Devices", sub: &[
        SubSelection { icon: None, label: "Processes" },
        SubSelection { icon: None, label: "Resources" },
        SubSelection { icon: None, label: "Sockets" }
    ] },
];

fn main() {
    println!("Hello, world!");
}
