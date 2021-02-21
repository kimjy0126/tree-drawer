use super::Node;
use super::Coord;

pub fn calc_pos(root: &mut Node) {
    calc_range(root);

    root.pos = Coord { x: 0.0, y: 0.0 };
    calc_pos_impl(root);
}

fn calc_range(node: &mut Node) -> u32 {
    let mut range: u32 = 0;
    for c in node.childs.iter_mut() {
        range += calc_range(c);
    }

    range = if range == 0 { 4 } else { range };
    node.range = range;
    range
}

fn calc_pos_impl(node: &mut Node) {
    let mut xpos = node.pos.x - node.range as f64 / 2.0;
    let ypos = node.pos.y + 4.0;
    
    for c in node.childs.iter_mut() {
        xpos += c.range as f64 / 2.0;
        c.pos = Coord { x: xpos, y: ypos };
        xpos += c.range as f64 / 2.0;
        calc_pos_impl(c);
    }
}
