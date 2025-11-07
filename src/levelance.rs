#[derive(Debug)]
pub struct LevelanceChunks {
    chunks: Vec<Chunk>,
    len: usize,
}

impl LevelanceChunks {
    fn new(chunks: Vec<Chunk>, len: usize) -> Self {
        Self { chunks, len }
    }

    pub fn decode(&self) -> Vec<i32> {
        self.chunks
            .iter()
            .map(|chunk| chunk.decode(self.len))
            .collect::<Vec<i32>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Chunk([Letter; 3]);

impl Chunk {
    // len is the length of the full string, minus dots
    fn decode(&self, len: usize) -> i32 {
        let letters = &self.0;
        let mut num = 0i32;
        let transform_0 = letters[0].decode(&mut num, 0, len as i32);
        let transform_1 = letters[1].decode(&mut num, 1, len as i32);
        let transform_2 = letters[2].decode(&mut num, 2, len as i32);
        let is_even = num % 2 == 0;
        apply_transform(&mut num, is_even, transform_0);
        apply_transform(&mut num, is_even, transform_1);
        apply_transform(&mut num, is_even, transform_2);
        num %= 10;
        num.abs()
    }
}

#[derive(Debug)]
enum Transform {
    Add5IfEven,
    Add1IfOdd,
    None,
}

fn apply_transform(num: &mut i32, is_even: bool, transform: Transform) {
    match transform {
        Transform::Add5IfEven => {
            if is_even {
                *num += 5;
            }
        }
        Transform::Add1IfOdd => {
            if !is_even {
                *num += 1;
            }
        }
        Transform::None => (),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Letter {
    // Pre-condition: idx must be in the range of [0,2]
    fn decode(&self, num: &mut i32, idx: i32, len: i32) -> Transform {
        // 1-indexed position in chunk
        let pos = idx + 1;
        let rev_pos = 3 - idx;
        let mut transform = Transform::None;
        match self {
            Letter::A => *num += 1,
            Letter::B => (),
            Letter::C => *num -= 1,
            Letter::D => *num *= 2,
            Letter::E => *num = *num * *num,
            Letter::F => {
                if idx == 0 {
                    *num -= 1;
                } else {
                    *num += 1;
                }
            }
            Letter::G => *num += 5,
            Letter::H => *num = 5,
            Letter::I => *num += pos,
            Letter::J => *num += len,
            Letter::K => *num -= len,
            Letter::L => *num = (pos + *num) - len,
            Letter::M => *num *= len,
            Letter::N => *num += 2,
            Letter::O => *num -= pos,
            Letter::P => *num += rev_pos,
            Letter::Q => *num -= rev_pos,
            Letter::R => *num += rev_pos + len,
            Letter::S => *num += rev_pos - len,
            Letter::T => *num += pos + 1,
            Letter::U => *num = *num + (rev_pos - len) + *num,
            Letter::V => *num -= 7,
            Letter::W => {
                *num += 1;
                transform = Transform::Add5IfEven;
            }
            Letter::X => {
                *num += 1;
                transform = Transform::Add1IfOdd;
            }
            Letter::Y => {
                if *num == 0 {
                    *num = 9;
                } else {
                    *num = 0;
                }
            }
            Letter::Z => *num = 0,
        };
        transform
    }
}

impl TryFrom<char> for Letter {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Letter::A),
            'B' => Ok(Letter::B),
            'C' => Ok(Letter::C),
            'D' => Ok(Letter::D),
            'E' => Ok(Letter::E),
            'F' => Ok(Letter::F),
            'G' => Ok(Letter::G),
            'H' => Ok(Letter::H),
            'I' => Ok(Letter::I),
            'J' => Ok(Letter::J),
            'K' => Ok(Letter::K),
            'L' => Ok(Letter::L),
            'M' => Ok(Letter::M),
            'N' => Ok(Letter::N),
            'O' => Ok(Letter::O),
            'P' => Ok(Letter::P),
            'Q' => Ok(Letter::Q),
            'R' => Ok(Letter::R),
            'S' => Ok(Letter::S),
            'T' => Ok(Letter::T),
            'U' => Ok(Letter::U),
            'V' => Ok(Letter::V),
            'W' => Ok(Letter::W),
            'X' => Ok(Letter::X),
            'Y' => Ok(Letter::Y),
            'Z' => Ok(Letter::Z),
            _ => Err(format!("Char {} was not a valid letter.", c)),
        }
    }
}

pub fn parse(input: &str) -> Result<LevelanceChunks, String> {
    let input = input.replace(".", "");
    let strlen = input.len();
    if strlen <= 5 {
        return Err(format!(
            "Length of input {} was too short to be a Levelance string",
            input
        ));
    }
    let end_idx_minus_1 = input.char_indices().nth_back(1).unwrap().0;
    if &input[0..=2] != "LPS" || &input[end_idx_minus_1..] != "LP" {
        return Err(format!(
            "Input {} is not a valid Levelance string because it does not start with LPS and end with LP",
            input
        ));
    }
    let levelance_str = &input[3..end_idx_minus_1];
    if !levelance_str.len().is_multiple_of(3) {
        return Err(format!(
            "Levelance string {} is not a valid Levelance string because its length is not divisible by 3",
            levelance_str
        ));
    }
    let mut letter_buf: [char; 3] = ['\0', '\0', '\0'];
    let mut chunks = vec![];
    for (idx, c) in levelance_str.char_indices() {
        let letter_idx = idx % 3;
        letter_buf[letter_idx] = c;
        if letter_idx == 2 {
            let letter_0 = letter_buf[0].try_into()?;
            let letter_1 = letter_buf[1].try_into()?;
            let letter_2 = letter_buf[2].try_into()?;
            chunks.push(Chunk([letter_0, letter_1, letter_2]));
        }
    }
    Ok(LevelanceChunks::new(chunks, strlen))
}

pub fn decode(input: &str) -> Result<Vec<i32>, String> {
    let chunks = parse(input)?;
    Ok(chunks.decode())
}
