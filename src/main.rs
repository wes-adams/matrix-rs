use std::{thread, time};

fn main() {
    let input: Option<&Vec<draw_tools::Object>> = Option::None;
    draw_tools::clear_screen();
    let _ = draw_tools::draw_cells(&vec![]);
    let mut output = inc_data::inc_data(input);

    loop {
        draw_tools::clear_screen();
        output = inc_data::inc_data(Some(&output));
        let _ = draw_tools::draw_cells(&output);
        let s = time::Duration::from_millis(100);
        thread::sleep(s);
    }
}
