use ibig::{ops::DivRem, ubig, UBig};

const ALPHABET: &[u8] = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".as_bytes();

/// Encode a byte array to a base58 string
pub fn encode(data: &[u8]) -> String {
    let mut count = 0;

    for d in data {
        if *d == 0 {
            count += 1;
        } else {
            break;
        }
    }
    let mut rem;
    let mut num = UBig::from_be_bytes(data);
    let prefix = "1".repeat(count);
    let mut enc = vec![];

    while num > ubig!(0) {
        (num, rem) = (&num).div_rem(58_usize);
        enc = [&[ALPHABET[rem]], enc.as_slice()].concat();
    }
    enc = [prefix.as_bytes(), enc.as_slice()].concat();

    String::from_utf8(enc).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_base58_1() {
        let data = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d")
            .unwrap();
        let expected = "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6";
        assert_eq!(encode(&data), expected);
    }

    #[test]
    fn test_base58_2() {
        let data =
            hex::decode("0000c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6")
                .unwrap();
        let expected = "11EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7";
        assert_eq!(encode(&data), expected);
    }
}
