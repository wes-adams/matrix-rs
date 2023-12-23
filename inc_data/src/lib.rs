static STR_LEN: u8 = 24;

pub fn inc_data(input: Option<&Vec<draw_tools::Object>>) -> Vec<draw_tools::Object> {
    if let Some(input) = input {
        // data already exists
        let mut output = input.to_vec();
        for o in &mut output {
            if o.y > draw_tools::get_max_y() + STR_LEN as i32 {
                o.c.pop();
            } else {
                if o.c.len() < STR_LEN.into() {
                    o.c.push(draw_tools::get_real_char());
                }
                o.y += 1;
            }
        }
        let obj = draw_tools::Object::new();
        output.push(obj);
        let obj = draw_tools::Object::new();
        output.push(obj);
        output
    } else {
        // Object vector is empty, init it
        let output = vec![draw_tools::Object::new()];
        output
    }
}
