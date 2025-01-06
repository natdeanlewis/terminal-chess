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
    format!("{}{}", COL_MAP[column], row)
}

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33
];

fn bit_scan(mut bit: u64) -> usize {
    let remainder: usize = (bit % 67) as usize;
    MOD67TABLE[remainder]
}

fn main() {
    let position = 1u64 << 4;
    println!("position: {}", position);
    for i in 0..64 {
        // println!("{}", index_to_position(i));
        let num = 1u64 << i;
        println!("{} -> {}", num, bit_scan(num));

    }
}
