extern crate piston_window;
extern crate find_folder;

use std::collections::VecDeque;
use piston_window::*;
use super::Node;

pub fn draw_tree(root: &Node) {
    let mut window: PistonWindow = WindowSettings::new(
        "Tree Drawer",
        [1600, 1000]
    ).exit_on_esc(true).build().unwrap();

    let mut glyphs;
    {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();
        glyphs = window.load_font(assets.join("NanumGothic.ttf")).unwrap();
    }

    window.set_lazy(true);

    draw_tree_impl(&mut window, &mut glyphs, root);
}

fn draw_tree_impl(window: &mut PistonWindow, fonts: &mut Glyphs, root: &Node) {
    while let Some(e) = window.next() {
        let mut node_q: VecDeque<&Node> = VecDeque::new();
        node_q.push_back(root.clone());

        window.draw_2d(&e, |c, g, device| {
            let r: f64 = 50.0;
            let transform = c.transform.trans(800.0, 10.0 + r);

            clear(color::WHITE, g);

            while !node_q.is_empty() {
                let node = node_q.pop_front().unwrap();
                for c in node.childs.iter() {
                    line(color::BLACK, 1.0,
                         [node.pos.x * r, node.pos.y * r, c.pos.x * r, c.pos.y * r],
                         transform, g);
                    node_q.push_back(c);
                }
                ellipse(color::WHITE,
                        rectangle::square((node.pos.x - 1.0) * r + 0.5, (node.pos.y - 1.0) * r + 0.5, r * 2.0 - 1.0),
                        transform, g);
                circle_arc(color::BLACK, 1.0, 0.0, f64::_360() as f64 * 1.2,
                        [(node.pos.x - 1.0) * r, (node.pos.y - 1.0) * r, r * 2.0, r * 2.0],
                        transform, g);
                text::Text::new(32).draw(
                    &node.content,
                    &mut *fonts,
                    &c.draw_state,
                    c.transform.trans(800.0 + node.pos.x * r, 10.0 + r + node.pos.y * r), g
                ).unwrap();

                fonts.factory.encoder.flush(device);
            }
        });
    }
}
