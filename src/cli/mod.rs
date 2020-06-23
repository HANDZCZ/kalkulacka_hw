use crate::types::*;
use crate::calculations::*;
use structopt::StructOpt;
use crate::utils::parse_hex;
use strum::VariantNames;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(StructOpt)]
#[structopt(name = "Kalkulačka!", about = "Kalkulačka pro pár operací s 64bit registry MM1 a MM2.")]
struct Opt {
    ///Stav registru MM1
    #[structopt(long, parse(try_from_str = parse_hex))]
    mm1: u64,
    ///Stav registru MM2
    #[structopt(long, parse(try_from_str = parse_hex), default_value = "0")]
    mm2: u64,
    ///Operace, která se má provést s registry
    #[structopt(short, long, possible_values = Operations::VARIANTS)]
    operation: Operations,
}

pub fn run() {
    let Opt { mm1, mm2, operation } = Opt::from_args();
    let mm1 = u64x1::new(mm1);
    let mm2 = u64x1::new(mm2);

    match calculate(mm1, mm2, &operation) {
        EitherRegisters::OneRegister(x) => println!("{}", x),
        EitherRegisters::TwoRegisters(x, y) => println!("MM1: {}\nMM2: {}", x, y),
    }
}