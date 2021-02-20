use std::io;

mod process_input; 
mod calculate_position;
mod draw_tree;

#[derive(Debug)]
struct Tree {
    root: Box<Node>,
}

#[derive(Debug)]
pub struct Node {
    content: String,
    range: u32,
    pos: (f64, f64),
    childs: Vec<Node>,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
//    input = String::from("[1[2[3][4]][5][6[7]]]");

    let mut tree: Tree = Tree { root: Box::new(process_input::build_tree(input)) };
    calculate_position::calc_pos(&mut tree.root);

    draw_tree::draw_tree(&tree.root);
}
