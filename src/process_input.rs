use std::convert::TryFrom;
use super::Node;
use super::Coord;

pub fn build_tree(input: String) -> Node {
    let input = input.trim();

    let mat: Vec<i32> = make_mat(&input);
    build_tree_impl(&input, &mat)
}

fn make_mat(input: &str) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut mat = Vec::new();

    for _ in 0..input.len() {
        mat.push(0);
    }

    for (i, &item) in input.as_bytes().iter().enumerate() {
        let i: i32 = i32::try_from(i).unwrap();
        if item == b'[' {
            stack.push(i);
        } else if item == b']' {
            let left_parenthesis = stack.pop().unwrap();
            mat[i as usize] = left_parenthesis - i;
            mat[left_parenthesis as usize] = i - left_parenthesis;
        }
    }

    mat
}

fn build_tree_impl(input: &str, mat: &[i32]) -> Node {
    let mut content = String::new();
    let mut childs: Vec<Node> = Vec::new();
    
    /* get content */
    {
        let mut flag_leaf = true;
        for (i, &item) in mat.iter().take(mat.len() - 1).skip(1).enumerate() {
            let i = i + 1;
            if item != 0 {
                flag_leaf = false;
                content = input[1..i].to_string();
                break;
            }
        }
        if flag_leaf {
            content = input[1..input.len() - 1].to_string();
        }
    }
    let content = content;

    /* get childs - recursion */
    {
        let inputlen: usize = input.len();
        let mut i: usize = 1;
        while i < inputlen {
            if mat[i] > 0 {
                childs.push(build_tree_impl(&input[i..(i + mat[i] as usize + 1)], &mat[i..(i + mat[i] as usize + 1)]));
                i += mat[i] as usize;
            } else {
                i += 1;
            }
        }
    }
    let childs = childs;

    Node { content, childs, range: 0, pos: Coord { x: 0.0, y: 0.0 } }
}
