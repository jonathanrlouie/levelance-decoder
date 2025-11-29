#[derive(Debug)]
pub struct LevelanceChunks {
    chunks: Vec<Chunk>,
    len: usize,
}

impl LevelanceChunks {
    fn new(chunks: Vec<Chunk>, len: usize) -> Self {
        Self { chunks, len }
    }

    pub fn decode(&self) -> Vec<String> {
        self.chunks
            .iter()
            .map(|chunk| chunk.decode(self.len))
            .collect::<Vec<String>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
// boolean field indicates if chunk is preceded by a dot
enum Chunk {
    NoDot([Letter; 3]),
    WithDot(Option<[Letter; 3]>),
}

fn decode_letters(letters: &[Letter; 3], len: usize) -> i32 {
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

impl Chunk {
    // len is the length of the full string, minus dots
    fn decode(&self, len: usize) -> String {
        match self {
            Chunk::NoDot(letters) => decode_letters(letters, len).to_string(),
            Chunk::WithDot(None) => ".".to_string(),
            Chunk::WithDot(Some(letters)) => format!(".{}", decode_letters(letters, len)),
        }
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
            Letter::U => *num = (rev_pos - len) + *num,
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
    let input_stripped = input.replace(".", "");
    let strlen = input_stripped.len();
    if strlen < 5 {
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
    let validations = levelance_str.split(".").map(|substr| {
        if !substr.len().is_multiple_of(3) {
            Err(format!(
                "Levelance substring {} is not a valid Levelance substring because its length is not divisible by 3",
                substr
            ))
        } else {
            Ok(())
        }
    }).collect::<Vec<Result<(), String>>>();
    if !validations.clone().into_iter().all(|res| res.is_ok()) {
        let mut err: String = String::new();
        for validation in validations {
            match validation {
                Ok(_) => (),
                Err(e) => err = format!("{}{}", err, e),
            }
        }
        return Err(err);
    }
    let mut letter_buf: Vec<char> = Vec::new();
    let mut chunks = vec![];
    let mut has_dot = false;
    for c in levelance_str.chars() {
        if c == '.' && !has_dot {
            has_dot = true;
        } else if c == '.' && has_dot {
            chunks.push(Chunk::WithDot(None))
        } else {
            letter_buf.push(c);
            if letter_buf.len() == 3 {
                let letter_0 = letter_buf[0].try_into()?;
                let letter_1 = letter_buf[1].try_into()?;
                let letter_2 = letter_buf[2].try_into()?;
                if has_dot {
                    chunks.push(Chunk::WithDot(Some([letter_0, letter_1, letter_2])));
                } else {
                    chunks.push(Chunk::NoDot([letter_0, letter_1, letter_2]));
                }
                letter_buf = Vec::new();
                has_dot = false;
            }
        }
    }
    if has_dot {
        chunks.push(Chunk::WithDot(None))
    }
    Ok(LevelanceChunks::new(chunks, strlen))
}

pub fn decode(input: &str) -> Result<String, String> {
    let chunks = parse(input)?;
    let decoded: Vec<String> = chunks.decode();
    Ok(decoded.join("").to_string())
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_AAA() {
        assert_eq!("3", decode("LPSAAALP").unwrap());
    }

    #[test]
    fn test_AXW() {
        assert_eq!("4", decode("LPSAXWLP").unwrap());
    }

    #[test]
    fn test_BBB() {
        assert_eq!("0", decode("LPSBBBLP").unwrap());
    }

    #[test]
    fn test_BGB() {
        assert_eq!("5", decode("LPSBGBLP").unwrap());
    }

    #[test]
    fn test_BBC() {
        assert_eq!("1", decode("LPSBBCLP").unwrap());
    }

    #[test]
    fn test_ADB() {
        assert_eq!("2", decode("LPSADBLP").unwrap());
    }

    #[test]
    fn test_GDC() {
        assert_eq!("9", decode("LPSGDCLP").unwrap());
    }

    #[test]
    fn test_AAE() {
        assert_eq!("4", decode("LPSAAELP").unwrap());
    }

    #[test]
    fn test_GAE() {
        assert_eq!("6", decode("LPSGAELP").unwrap());
    }

    #[test]
    fn test_FBB() {
        assert_eq!("1", decode("LPSFBBLP").unwrap());
    }

    #[test]
    fn test_BFB() {
        assert_eq!("1", decode("LPSBFBLP").unwrap());
    }

    #[test]
    fn test_BHA() {
        assert_eq!("6", decode("LPSBHALP").unwrap());
    }

    #[test]
    fn test_BHH() {
        assert_eq!("5", decode("LPSBHHLP").unwrap());
    }

    #[test]
    fn test_GAH() {
        assert_eq!("5", decode("LPSGAHLP").unwrap());
    }

    #[test]
    fn test_IBB() {
        assert_eq!("1", decode("LPSIBBLP").unwrap());
    }

    #[test]
    fn test_BIG() {
        assert_eq!("7", decode("LPSBIGLP").unwrap());
    }

    #[test]
    fn test_BBI() {
        assert_eq!("3", decode("LPSBBILP").unwrap());
    }

    #[test]
    fn test_JJJ() {
        assert_eq!("4", decode("LPSJJJLP").unwrap());
    }

    #[test]
    fn test_JJJBBB() {
        assert_eq!("30", decode("LPSJJJBBBLP").unwrap());
    }

    #[test]
    fn test_KJB() {
        assert_eq!("0", decode("LPSKJBLP").unwrap());
    }

    #[test]
    fn test_AKB() {
        assert_eq!("7", decode("LPSAKBLP").unwrap());
    }

    #[test]
    fn test_BAL() {
        assert_eq!("4", decode("LPSBALLP").unwrap());
    }

    #[test]
    fn test_AAM() {
        assert_eq!("6", decode("LPSAAMLP").unwrap());
    }

    #[test]
    fn test_AAN() {
        assert_eq!("4", decode("LPSAANLP").unwrap());
    }

    #[test]
    fn test_NAN() {
        assert_eq!("5", decode("LPSNANLP").unwrap());
    }

    #[test]
    fn test_BBO() {
        assert_eq!("3", decode("LPSBBOLP").unwrap());
    }

    #[test]
    fn test_AAO() {
        assert_eq!("1", decode("LPSAAOLP").unwrap());
    }

    #[test]
    fn test_GOB() {
        assert_eq!("3", decode("LPSGOBLP").unwrap());
    }

    #[test]
    fn test_PAN() {
        assert_eq!("6", decode("LPSPANLP").unwrap());
    }

    #[test]
    fn test_NGP() {
        assert_eq!("8", decode("LPSNGPLP").unwrap());
    }

    #[test]
    fn test_BPB() {
        assert_eq!("2", decode("LPSBPBLP").unwrap());
    }

    #[test]
    fn test_BPQ() {
        assert_eq!("1", decode("LPSBPQLP").unwrap());
    }

    #[test]
    fn test_QGB() {
        assert_eq!("2", decode("LPSQGBLP").unwrap());
    }

    #[test]
    fn test_RAR() {
        assert_eq!("1", decode("LPSRARLP").unwrap());
    }

    #[test]
    fn test_SAS() {
        assert_eq!("1", decode("LPSSASLP").unwrap());
    }

    #[test]
    fn test_AAR() {
        assert_eq!("1", decode("LPSAARLP").unwrap());
    }

    #[test]
    fn test_BBT() {
        assert_eq!("4", decode("LPSBBTLP").unwrap());
    }

    #[test]
    fn test_BTB() {
        assert_eq!("3", decode("LPSBTBLP").unwrap());
    }

    #[test]
    fn test_TBB() {
        assert_eq!("2", decode("LPSTBBLP").unwrap());
    }

    #[test]
    fn test_ABU() {
        assert_eq!("6", decode("LPSABULP").unwrap());
    }

    #[test]
    fn test_NUB() {
        assert_eq!("4", decode("LPSNUBLP").unwrap());
    }

    #[test]
    fn test_GNV() {
        assert_eq!("0", decode("LPSGNVLP").unwrap());
    }

    #[test]
    fn test_WWB() {
        assert_eq!("2", decode("LPSWWBLP").unwrap());
    }

    #[test]
    fn test_XXB() {
        assert_eq!("2", decode("LPSXXBLP").unwrap());
    }

    #[test]
    fn test_XAX() {
        assert_eq!("5", decode("LPSXAXLP").unwrap());
    }

    #[test]
    fn test_XBB() {
        assert_eq!("2", decode("LPSXBBLP").unwrap());
    }

    #[test]
    fn test_XXA() {
        assert_eq!("5", decode("LPSXXALP").unwrap());
    }

    #[test]
    fn test_WWA() {
        assert_eq!("3", decode("LPSWWALP").unwrap());
    }

    #[test]
    fn test_WAB() {
        assert_eq!("7", decode("LPSWABLP").unwrap());
    }

    #[test]
    fn test_WXB() {
        assert_eq!("7", decode("LPSWXBLP").unwrap());
    }

    #[test]
    fn test_GNY() {
        assert_eq!("0", decode("LPSGNYLP").unwrap());
    }

    #[test]
    fn test_YYN() {
        assert_eq!("2", decode("LPSYYNLP").unwrap());
    }

    #[test]
    fn test_AYY() {
        assert_eq!("9", decode("LPSAYYLP").unwrap());
    }

    #[test]
    fn test_YVA() {
        assert_eq!("3", decode("LPSYVALP").unwrap());
    }

    #[test]
    fn test_GNZ() {
        assert_eq!("0", decode("LPSGNZLP").unwrap());
    }

    #[test]
    fn test_ZZZ() {
        assert_eq!("0", decode("LPSZZZLP").unwrap());
    }

    #[test]
    fn test_empty() {
        assert_eq!("", decode("LPSLP").unwrap());
    }

    #[test]
    fn test_empty_with_dots() {
        assert_eq!("...", decode("LPS...LP").unwrap());
    }

    #[test]
    fn test_empty_with_dot() {
        assert_eq!(".", decode("LPS.LP").unwrap());
    }

    #[test]
    fn test_AAA_with_dots() {
        assert_eq!(".3.", decode("LPS.AAA.LP").unwrap());
    }

    #[test]
    fn test_AAA_with_extra_dots() {
        assert_eq!(".3..", decode("LPS.AAA..LP").unwrap());
    }

    #[test]
    fn test_AAA_dot_BBB() {
        assert_eq!("3.0", decode("LPSAAA.BBBLP").unwrap());
    }
}
