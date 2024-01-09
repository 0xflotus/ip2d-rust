use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("ip2d")
        .author("0xflotus")
        .version("0.4.0")
        .about("A converter for IPv4 Addresses")
        .arg(
            Arg::new("ip")
                .help("Converts an IPv4 Address to an integer")
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .value_name("number")
                .help("Converts an integer to an IPv4 Address"),
        )
        .arg(
            Arg::new("hex")
                .short('x')
                .long("hex")
                .action(ArgAction::SetTrue)
                .help("Converts an IPv4 Address to a hex number"),
        )
        .get_matches();

    if let Some(number) = matches.get_one::<String>("reverse") {
        let num = number.parse::<u32>().unwrap();
        let vec: Vec<String> = vec![0x18, 0x10, 0x8, 0x0]
            .into_iter()
            .map(|x| (num >> x) & 0xff)
            .map(|x| x.to_string())
            .collect();
        println!("{}", vec.join("."));
        return;
    }

    if let Some(ip) = matches.get_one::<String>("ip") {
        let splitted = ip
            .split(".")
            .map(|x| x.parse::<u32>().unwrap())
            .fold(0, |x, y| (x << 0x8) | y);
        if matches.get_flag("hex") {
            println!("{:#010x}", splitted);
        } else {
            println!("{}", splitted);
        }
    } else {
        println!("Please specify an IPv4 address to convert.");
    }
}
