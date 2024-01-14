pub fn to_u32(ip: &str) -> u32 {
    ip.split(".")
        .map(|x| x.parse::<u32>().unwrap())
        .fold(0, |x, y| (x << 0x8) | y)
}

pub fn to_str(num: u32) -> String {
    let vec: Vec<String> = vec![0x18, 0x10, 0x8, 0x0]
            .into_iter()
            .map(|x| ((num >> x) & 0xff).to_string())
            .collect();
    return vec.join(".");
}

#[cfg(test)]
mod tests {
    use super::{to_u32, to_str};

    #[test]
    fn u32_works_with_255_0_255_0() {
        let result = to_u32("255.0.255.0");
        assert_eq!(result, 4278255360);
    }

    #[test]
    fn u32_works_with_0_0_0_1() {
        let result = to_u32("0.0.0.1");
        assert_eq!(result, 1);
    }

    #[test]
    fn u32_works_with_255_255_255_255() {
        let result = to_u32("255.255.255.255");
        assert_eq!(result, 0xFFFFFFFF);
    }

    #[test]
    fn str_works_with_4278255360() {
        let result = to_str(4278255360);
        assert_eq!(result, "255.0.255.0");
    }

    #[test]
    fn str_works_with_1() {
        let result = to_str(1);
        assert_eq!(result, "0.0.0.1");
    }

    #[test]
    fn str_works_with_4294967295() {
        let result = to_str(4294967295);
        assert_eq!(result, "255.255.255.255");
    }
}
