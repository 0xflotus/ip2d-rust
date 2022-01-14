use clap::{App, Arg};

fn main() {
    let matches = App::new("ip2d")
        .version("0.2.0")
        .about("A converter for IPv4 Addresses")
        .arg(
            Arg::new("ip")
                .help("Converts an IPv4 Address to a number")
                .required(false)
                .index(1_usize),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .takes_value(true)
                .value_name("number")
                .help("Converts a number to an IPv4 Address"),
        )
        .arg(
            Arg::new("hex")
                .short('x')
                .long("hex")
                .takes_value(false)
                .help("Converts an IPv4 Address to a hex number"),
        )
        .get_matches();

    if matches.is_present("reverse") {
        if let Some(number) = matches.value_of("reverse") {
            let num = number.parse::<u32>().unwrap();
            let vec: Vec<String> = vec![0x18, 0x10, 0x8, 0x0]
                .into_iter()
                .map(|x| (num >> x) & 0xff)
                .map(|x| x.to_string())
                .collect();
            println!("{}", vec.join("."));
        }
        return;
    }

    if let Some(ip) = matches.value_of("ip") {
        let splitted = ip
            .split(".")
            .map(|x| x.parse::<u32>().unwrap())
            .fold(0, |x, y| (x << 0x8) | y);
        if matches.is_present("hex") {
            println!("{:#010x}", splitted);
        } else {
            println!("{}", splitted);
        }
    } else {
        println!("Please specify an IPv4 address to convert.");
    }
}
