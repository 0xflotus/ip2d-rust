use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use ip2d::{to_integer, to_str};

fn main() -> Result<()> {
    let matches = Command::new("ip2d")
        .author("0xflotus")
        .version("0.5.0")
        .about("A converter for IP Addresses")
        .arg(Arg::new("ip").help("Converts an IP Address to an integer"))
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .value_name("number")
                .help("Converts an integer to an IP Address"),
        )
        .arg(
            Arg::new("hex")
                .short('x')
                .long("hex")
                .action(ArgAction::SetTrue)
                .help("Converts an IP Address to a hex number"),
        )
        .arg(
            Arg::new("v6")
                .short('6')
                .long("v6")
                .action(ArgAction::SetTrue)
                .help("Force print an IPv6"),
        )
        .get_matches();

    if let Some(number) = matches.get_one::<String>("reverse") {
        let num = number.parse::<u128>()?;
        if matches.get_flag("v6") {
            println!("{}", to_str(num, true));
        } else {
            println!("{}", to_str(num, false));
        }
        return Ok(());
    }

    if let Some(ip) = matches.get_one::<String>("ip") {
        if let Ok(splitted) = to_integer(ip) {
            if matches.get_flag("hex") {
                if matches.get_flag("v6") {
                    println!("{:#034x}", splitted);   
                } else {
                    println!("{:#010x}", splitted);
                }
            } else {
                println!("{}", splitted);
            }
        } else {
            println!("An invalid IP was provided.")
        }
    } else {
        println!("Please specify an IP address to convert.");
    }

    Ok(())
}
