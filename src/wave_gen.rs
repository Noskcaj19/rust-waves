use pancurses as pc;


const TOP_BAR: &'static [char] = &['⣀', '⣤', '⣶', '⣿'];
const LOWER_BAR: &'static [char] = &['⠉', '⠛', '⠿', '⣿'];

pub fn gen_row<'a>(value: u16) -> Vec<char> {
    let value = value as usize;
    // TODO: Implement with capacity
    let mut out = Vec::new();
    match value {
        0 => {}
        1...4 => {
            out.push(TOP_BAR[value - 1]);
            out.push(LOWER_BAR[value - 1]);
        }
        _ => {
            let num_full = value / 4;
            let remainder = (value - 1) % 4;
            // Add remainder top
            if remainder != 3 {
                out.push(TOP_BAR[remainder]);
            }

            // Add "upper middle"
            for _ in 0..num_full {
                out.push(TOP_BAR[3])
            }
            // Add "lower middle"
            for _ in 0..num_full {
                out.push(LOWER_BAR[3])
            }

            // Add remainder bottom
            if remainder != 3 {
                out.push(LOWER_BAR[remainder]);
            }
        }
    };
    out
}

pub fn draw_row(window: &pc::Window, row: Vec<char>, column: i32, lines: i32) {
    let axis = lines / 2;
    // How far from the horizontal axis to start
    let offset = (row.len() / 2) as i32;

    for (i, val) in (axis - offset..axis + offset).enumerate() {
        window.mvaddstr(val, column, &row[i as usize].to_string());
    }
    window.refresh();
}
