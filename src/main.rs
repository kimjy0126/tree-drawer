use std::io;

mod process_input; 
mod calculate_position;
mod draw_tree;

struct Tree {
    root: Box<Node>,
}

pub struct Node {
    content: String,
    range: u32,
    pos: Coord,
    childs: Vec<Node>,
}

pub struct Coord {
    x: f64,
    y: f64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut tree: Tree = Tree { root: Box::new(process_input::build_tree(input)) };
    calculate_position::calc_pos(&mut tree.root);

    draw_tree::draw_tree(&tree.root);
}
