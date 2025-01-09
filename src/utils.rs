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

#[allow(dead_code)]
pub fn bit_to_coords(bit: u64) -> Result<String, String> {
    if bit == 0 {
        Err("No piece present!".to_string())
    } else {
        let onebit_index = bit_to_onebit_index(bit);
        Ok(onebit_index_to_coords(onebit_index))
    }
}

pub fn coords_to_bit(coords: &str) -> Result<u64, String> {
    if let Ok(onebit_index) = coords_to_onebit_index(coords) {
        return Ok(onebit_index_to_bit(onebit_index))
    }
    return Err(format!("Invalid coords: {}", coords));
}

pub fn coords_to_onebit_index(coords: &str) -> Result<usize, String> {
    if coords.len() != 2 {
        return Err(format!("Invalid length: {}, string: '{}'", coords.len(), coords));
    }

    let bytes = coords.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97 + 8 {
        return Err(format!("Invalid column character: {}, string: '{}'", byte0 as char, coords));
    }
    let column = (byte0 - 97) as u32;

    let byte1 = bytes[1];
    let row;

    match (byte1 as char).to_digit(10) {
        Some(number) => if number < 1 || number > 8 {
            return Err(format!("Invalid row character: {}, string: '{}'", byte1, coords));
        } else {
            row = number - 1
        },
        None => return Err(format!("Invalid row character: {}, string: '{}'", byte1, coords)),
    }
    let onebit_index = row * 8 + column;
    Ok(onebit_index as usize)
}

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn onebit_index_to_coords(onebit_index: usize) -> String {
    let column = onebit_index % 8;
    let row = onebit_index / 8 + 1;
    format!("{}{}", COL_MAP[column], row)
}

pub fn onebit_index_to_bit(onebit_index: usize) -> u64 {
    1u64 << onebit_index
}

pub fn bit_to_onebit_index(bit: u64) -> usize {
    let remainder: usize = (bit % 67) as usize;
    MOD67TABLE[remainder]
}

pub fn split_on(s: &str, sep: char) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == sep {
            return (&s[0..i], &s[i + 1..]);
        }
    }
    (&s[..], "")
}