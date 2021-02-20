use super::Node;

pub fn calc_pos(root: &mut Node) {
    calc_range(root);

    root.pos = (0.0, 0.0);
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
    let rpos = node.pos.0 + 4.0;
    let mut cpos = node.pos.1 - node.range as f64 / 2.0;
    
    for c in node.childs.iter_mut() {
        cpos += c.range as f64 / 2.0;
        c.pos = (rpos, cpos);
        cpos += c.range as f64 / 2.0;
        calc_pos_impl(c);
    }
}
