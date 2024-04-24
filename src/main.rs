use clap::Parser;
use std::net::Ipv4Addr;
use whois_rust::{WhoIs, WhoIsLookupOptions};


#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "whois-rust is a cli tool that will interact with whois api to query your domain / ip address")]

//https://users.rust-lang.org/t/how-to-write-two-conflict-options-with-structopt-in-cli-app/48050/5
struct Args {
    //domain name 
    #[arg(short, long)]
    domain: Option<String>,


    //https://stackoverflow.com/questions/71693683/rust-clap-parse-ipv4addr
    // Ip address
    #[arg(short, long, conflicts_with = "domain")]
    ip: Option<Ipv4Addr>,

}

fn main() {
    let args = Args::parse();
    let whois = WhoIs::from_path("./server.json").unwrap();

    // let result: String = whois.lookup(WhoIsLookupOptions::from_string("magiclen.org").unwrap()).unwrap();
    // println!("{}", result)

    if let Some(domain) = &args.domain {
        let result: String = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()).unwrap();
        println!("Domain: {}", result);
    }
    else if let Some(ip) = &args.ip {
        //whois lookup takes string
        let ip_str = ip.to_string();
        let result: String = whois.lookup(WhoIsLookupOptions::from_string(ip_str).unwrap()).unwrap();
        println!("Domain: {}", result);
    } else {
        println!("Please specify either --domain or --ip");
    }
}




