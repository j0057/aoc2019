pub static CLEAR_SCREEN: &[u8] = b"\x1b[2J";

pub fn move_cursor(row: i32, column: i32) -> Vec<u8> {
    format!("\x1b[{};{}H", row, column)
        .as_bytes()
        .to_vec()
}
