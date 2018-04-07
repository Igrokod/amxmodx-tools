use std::char;
use std::ops::Range;

pub trait DebugU8 {
    const PRINTABLE_RANGE: Range<u8> = 0x20..0x7E;
    fn printable(&self) -> String;
}

impl DebugU8 for [u8] {
    fn printable(&self) -> String {
        self.iter()
            .map(|&c| if c >= Self::PRINTABLE_RANGE.start &&
                c <= Self::PRINTABLE_RANGE.end
            {
                format!("{}", char::from(c))
            } else {
                format!("\\x{:02x}", c)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::DebugU8;

    #[test]
    fn it_return_printable_representation_of_characters() {
        assert_eq!("|||\\x00\\xff|", [124, 124, 124, 0, 255, 124].printable());
        assert_eq!("\\x00hel\\x00lo", "\0hel\0lo".as_bytes().printable());
    }
}
