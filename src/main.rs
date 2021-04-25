use std::fmt;

type Temperature = f32;

#[derive(Debug)]
pub struct Atom {
    no: u8,
    sym: &'static str,
    name: &'static str,
    p: u8,
    ve: u8,
    wt: f32,
    mp: Temperature,
    bp: Temperature,
    d: f32,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:3}. {:3}: {}", self.no, self.sym, self.name)
    }
}

#[rustfmt::skip]
pub const PERIODIC_TABLE: [Atom; 18] = [
    Atom { no: 1u8, sym: "H" , name: "Hydrogen" , p: 1u8, ve: 1u8, wt:  1.00798 , mp: -259.14, bp: -252.87 , d: 0.00008988 },
    Atom { no: 2u8, sym: "He", name: "Helium"   , p: 2u8, ve: 0u8, wt:  4.002602, mp: -272.2 , bp: -268.934, d: 0.00001785 },
    Atom { no: 3u8, sym: "Li", name: "Lithium"  , p: 3u8, ve: 1u8, wt:  6.968   , mp:  180.54, bp: 1347.0  , d: 0.534      },
    Atom { no: 4u8, sym: "Be", name: "Beryllium", p: 4u8, ve: 2u8, wt:  9.01218 , mp: 1285.0 , bp: 2780.0  , d: 1.857      },
    Atom { no: 5u8, sym: "B" , name: "Boron"    , p: 5u8, ve: 3u8, wt: 10.806   , mp: 2300.0 , bp: 3658.0  , d: 2.34       },
    Atom { no: 6u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 7u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    Atom { no: 0u8, sym: "", name: ""  , p: 0u8, ve: 0u8, wt: 0.0, mp: 0.0, bp: 0.0, d: 0.0 },
    //Atom { no: u8, sym: "", name: ""  , p: u8, ve: u8, wt: , mp: , bp: , d:  },
    //Atom { no: u8, sym: "", name: ""  , p: u8, ve: u8, wt: , mp: , bp: , d:  },
];

fn main() {
    for atom in PERIODIC_TABLE.iter() {
        println!("{:?}", atom);
    }
}
