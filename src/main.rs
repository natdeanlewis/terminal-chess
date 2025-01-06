use std::ptr::write;

type PiecePosition = u64;

// fn bit_to_positon(bit: PiecePosition) -> Result<String, String> {
//     if bit == 0 {
//         return Err("No piece present!".to_string());
//     } else {
//         let onebit_index = bit_scan(bit);
//         return Ok(index_to_position(onebit_index));
//     }
// }

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;
    return format!("{}{}", COL_MAP[column], row);
}

fn main() {
    let position = 1u64 << 4;
    println!("position: {}", position);
    for i in 0..64 {
        println!("{}", index_to_position(i));
    }
}
