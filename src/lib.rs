use std::net::IpAddr::{self, *};
use std::net::Ipv6Addr;

use anyhow::Result;

pub fn to_integer(ip: &str) -> Result<u128> {
    let addr: IpAddr = ip.parse()?;
    match addr {
        V4(a) => {
            let res: u32 = a.into();
            Ok(res as u128)
        }
        V6(a) => Ok(a.into()),
    }
}

pub fn to_str(num: u128) -> String {
    let addr = Ipv6Addr::from(num);
    // Try and see if we can return an IPv4 address, otherwise fallback to IPv6.
    match addr.to_ipv4() {
        Some(a) => a.to_string(),
        None => addr.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{to_integer, to_str};

    #[test]
    fn to_integer_works_with_255_0_255_0() {
        let result = to_integer("255.0.255.0").unwrap();
        assert_eq!(result, 4278255360);
    }

    #[test]
    fn to_integer_works_with_0_0_0_1() {
        let result = to_integer("0.0.0.1").unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn to_integer_works_with_255_255_255_255() {
        let result = to_integer("255.255.255.255").unwrap();
        assert_eq!(result, 0xFFFFFFFF);
    }

    #[test]
    fn to_integer_works_with_ipv6() {
        let cases = vec!["::1", "2001:4860:4860::8888", "2606:4700:4700::1111"];
        let expected = vec![
            1,
            42541956123769884636017138956568135816,
            50543257694033307102031451402929180945,
        ];
        for (i, case) in cases.iter().enumerate() {
            let result = to_integer(case).unwrap();
            assert_eq!(result, expected[i]);
        }
    }

    #[test]
    fn to_integer_parse_failure() {
        let cases = vec!["256.0.0.0", "1::::1"];
        for case in cases {
            assert!(to_integer(case).is_err());
        }
    }

    #[test]
    fn str_works_with_ipv6() {
        let result = to_str(50543257694033307102031451402929180945);
        assert_eq!(result, "2606:4700:4700::1111");
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
