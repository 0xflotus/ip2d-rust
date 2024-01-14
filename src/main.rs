use clap::{Arg, ArgAction, Command};
use ip2d::{to_u32, to_str};

fn main() {
    let matches = Command::new("ip2d")
        .author("0xflotus")
        .version("0.5.0")
        .about("A converter for IPv4 Addresses")
        .arg(Arg::new("ip").help("Converts an IPv4 Address to an integer"))
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
        println!("{}", to_str(num));
        return;
    }

    if let Some(ip) = matches.get_one::<String>("ip") {
        let splitted = to_u32(ip);
        if matches.get_flag("hex") {
            println!("{:#010x}", splitted);
        } else {
            println!("{}", splitted);
        }
    } else {
        println!("Please specify an IPv4 address to convert.");
    }
}
