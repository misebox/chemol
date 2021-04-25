use std::fmt;

type Temperature = f32;

#[derive(Debug)]
pub struct Atom {
    // Atomic number
    no: u8,
    // Chemical symbol
    sym: &'static str,
    // Element name
    name: &'static str,
    // number of Proton
    p: u8,
    // number of Valence electron
    ve: u8,
    // Atomic weight
    wt: f32,
    // Melting point
    mp: Temperature,
    // Boiling point
    bp: Temperature,
    // Density
    d: f32,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:3}. {:3}: {}", self.no, self.sym, self.name)
    }
}

#[rustfmt::skip]
pub const PERIODIC_TABLE: [Atom; 18] = [
    Atom { no:  1u8, sym: "H" , name: "Hydrogen"  , p:  1u8, ve: 1u8, wt:  1.00798    , mp: -259.14 , bp: -252.87 , d: 0.00008988 },
    Atom { no:  2u8, sym: "He", name: "Helium"    , p:  2u8, ve: 0u8, wt:  4.002602   , mp: -272.2  , bp: -268.934, d: 0.00001785 },
    Atom { no:  3u8, sym: "Li", name: "Lithium"   , p:  3u8, ve: 1u8, wt:  6.968      , mp:  180.54 , bp: 1347.0  , d: 0.534      },
    Atom { no:  4u8, sym: "Be", name: "Beryllium" , p:  4u8, ve: 2u8, wt:  9.01218    , mp: 1285.0  , bp: 2780.0  , d: 1.857      },
    Atom { no:  5u8, sym: "B" , name: "Boron"     , p:  5u8, ve: 3u8, wt: 10.814      , mp: 2300.0  , bp: 3658.0  , d: 2.34       },
    Atom { no:  6u8, sym: "C" , name: "Carbon"    , p:  6u8, ve: 4u8, wt: 12.0106     , mp: 3550.0  , bp: 4800.0  , d: 3.513      },
    Atom { no:  7u8, sym: "N" , name: "Nitrogen"  , p:  7u8, ve: 5u8, wt: 14.00686    , mp: -209.86 , bp: -195.8  , d: 0.0012506  },
    Atom { no:  8u8, sym: "O" , name: "Oxygen"    , p:  8u8, ve: 6u8, wt: 15.99940    , mp: -218.4  , bp: -182.96 , d: 0.001429   },
    Atom { no:  9u8, sym: "F" , name: "Fluorine"  , p:  9u8, ve: 7u8, wt: 18.998403163, mp: -219.62 , bp: -188.14 , d: 0.001696   },
    Atom { no: 10u8, sym: "Ne", name: "Neon"      , p: 10u8, ve: 0u8, wt: 20.1797     , mp: -248.67 , bp: -246.05 , d: 0.0008999  },
    Atom { no: 11u8, sym: "Na", name: "Sodium"    , p: 11u8, ve: 1u8, wt: 22.98976928 , mp:   97.81 , bp:  883.0  , d: 0.971      },
    Atom { no: 12u8, sym: "Mg", name: "Magnesium" , p: 12u8, ve: 2u8, wt: 24.306      , mp:  648.8  , bp: 1090.0  , d: 1.738      },
    Atom { no: 13u8, sym: "Al", name: "Aluminium" , p: 13u8, ve: 3u8, wt: 26.9815385  , mp:  660.32 , bp: 2467.0  , d: 2.6989     },
    Atom { no: 14u8, sym: "Si", name: "Silicon"   , p: 14u8, ve: 4u8, wt: 28.085      , mp: 1410.0  , bp: 2355.0  , d: 2.3296     },
    Atom { no: 15u8, sym: "P" , name: "Phosphorus", p: 15u8, ve: 5u8, wt: 30.973761998, mp:   44.2  , bp:  280.0  , d: 1.82       },
    Atom { no: 16u8, sym: "S" , name: "Sulfur"    , p: 16u8, ve: 6u8, wt: 32.068      , mp:  115.9  , bp:  444.674, d: 2.014      },
    Atom { no: 17u8, sym: "Cl", name: "Chlorine"  , p: 17u8, ve: 7u8, wt: 35.452      , mp: -101.0  , bp:  -33.97 , d: 0.003214   },
    Atom { no: 18u8, sym: "Ar", name: "Argon"     , p: 18u8, ve: 0u8, wt: 39.948      , mp: -189.3  , bp: -185.8  , d: 0.001784   },
    //Atom { no: u8, sym: "", name: ""  , p: u8, ve: u8, wt: , mp: , bp: , d:  },
    //Atom { no: u8, sym: "", name: ""  , p: u8, ve: u8, wt: , mp: , bp: , d:  },
];

fn main() {
    for atom in PERIODIC_TABLE.iter() {
        println!("{:?}", atom);
    }
}
