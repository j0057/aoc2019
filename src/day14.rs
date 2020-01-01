use std::collections::HashMap;

//
// enum InputError
//

#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Error parsing text: {0:?}")]
    Parse(Option<String>),

    #[error("Error parsing number: {0:?}")]
    ParseInt(#[from] std::num::ParseIntError),
}

//
// struct Reagent
//

#[derive(Debug, Clone, PartialEq)]
pub struct Reagent {
    quantity: i64,
    name: String
}

impl Reagent {
    fn new(qty: i64, name: &str) -> Self {
        Self { quantity: qty, name: name.into() }
    }
}

impl std::str::FromStr for Reagent {
    type Err = InputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts = text.split(' ').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(InputError::Parse(Some(text.to_owned())));
        }
        let result = Reagent::new(parts[0].parse()?, parts[1]);
        Ok(result)
    }
}

//
// struct Reaction
//

#[derive(Debug, Clone, PartialEq)]
pub struct Reaction {
    precursors: Vec<Reagent>,
    product: Reagent,
}

impl Reaction {
    fn new(precursors: Vec<Reagent>, product: Reagent) -> Self {
        Self { precursors, product }
    }
}

impl std::str::FromStr for Reaction {
    type Err = InputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts = text.split(" => ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(InputError::Parse(Some(text.to_owned())));
        }
        let result = Reaction::new(
            parts[0]
                .split(", ")
                .map(|s| s.parse::<Reagent>())
                .collect::<Result<Vec<Reagent>, InputError>>()?,
            parts[1].parse::<Reagent>()?);
        Ok(result)
    }
}

//
// struct Reactor
//

struct Reactor<'a> {
    reactions: HashMap<&'a str, &'a Reaction>,
    surplus: HashMap<String, i64>,
}

impl<'a> Reactor<'a> {
    fn new(reactions: &'a [Reaction]) -> Self {
        Self {
            reactions: Self::index_reactions(reactions),
            surplus: HashMap::new(),
        }
    }

    fn index_reactions(reactions: &'a [Reaction]) -> HashMap<&'a str, &'a Reaction> {
        let mut result = HashMap::<&str, &Reaction>::new();
        for reaction in reactions {
            result.insert(&reaction.product.name, &reaction);
        }
        result
    }

    fn ore_cost(&mut self, qty: i64, tgt: &'a str) -> i64 {
        if tgt == "ORE" {
            return qty;
        }
        let reaction = self.reactions.get(tgt).unwrap();
        let count = (qty as f64 / reaction.product.quantity as f64).ceil() as i64;
        if count * reaction.product.quantity > qty {
            *self.surplus.entry(tgt.to_owned()).or_insert(0) += count * reaction.product.quantity - qty;
        }
        reaction.precursors
            .iter()
            .map(|reagent| {
                let needed = count * reagent.quantity;
                let surplus = self.surplus.remove(&reagent.name).unwrap_or(0);
                if needed < surplus {
                    self.surplus.insert(reagent.name.to_owned(), surplus - needed); 0
                } else {
                    self.ore_cost(needed - surplus, &reagent.name)
                }
            })
            .sum()
    }
}

//
// solution
//

pub fn day14a(reactions: &[Reaction]) -> i64 {
    Reactor::new(reactions).ore_cost(1, "FUEL")
}

//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    static EX1: &str = "10 ORE => 10 A\n\
                        1 ORE => 1 B\n\
                        7 A, 1 B => 1 C\n\
                        7 A, 1 C => 1 D\n\
                        7 A, 1 D => 1 E\n\
                        7 A, 1 E => 1 FUEL";

    #[test]
    fn test_14_ex1() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("10 ORE => 10 A\n\
                                       1 ORE => 1 B\n\
                                       7 A, 1 B => 1 C\n\
                                       7 A, 1 C => 1 D\n\
                                       7 A, 1 D => 1 E\n\
                                       7 A, 1 E => 1 FUEL")?;
        assert_eq!(input[0], super::Reaction::new(vec![super::Reagent::new(10, "ORE")], super::Reagent::new(10, "A")));
        let mut reactor = super::Reactor::new(&input);
        assert_eq!(reactor.ore_cost(1, "FUEL"), 31);
        assert_eq!(reactor.surplus.get("A"), Some(&2));
        Ok(())
    }

    #[test]
    fn test_14_ex2() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("9 ORE => 2 A\n\
                                       8 ORE => 3 B\n\
                                       7 ORE => 5 C\n\
                                       3 A, 4 B => 1 AB\n\
                                       5 B, 7 C => 1 BC\n\
                                       4 C, 1 A => 1 CA\n\
                                       2 AB, 3 BC, 4 CA => 1 FUEL")?;
        let mut reactor = super::Reactor::new(&input);
        assert_eq!(reactor.ore_cost(1, "FUEL"), 165);
        Ok(())
    }

    #[test]
    fn test_14_ex3() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("157 ORE => 5 NZVS\n\
                                       165 ORE => 6 DCFZ\n\
                                       44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
                                       12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
                                       179 ORE => 7 PSHF\n\
                                       177 ORE => 5 HKGWZ\n\
                                       7 DCFZ, 7 PSHF => 2 XJWVT\n\
                                       165 ORE => 2 GPVTF\n\
                                       3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")?;
        let mut reactor = super::Reactor::new(&input);
        assert_eq!(reactor.ore_cost(1, "FUEL"), 13312);
        Ok(())
    }

    #[test]
    fn test_14_ex4() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
                                       17 NVRVD, 3 JNWZP => 8 VPVL\n\
                                       53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
                                       22 VJHF, 37 MNCFX => 5 FWMGM\n\
                                       139 ORE => 4 NVRVD\n\
                                       144 ORE => 7 JNWZP\n\
                                       5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
                                       5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
                                       145 ORE => 6 MNCFX\n\
                                       1 NVRVD => 8 CXFTF\n\
                                       1 VJHF, 6 MNCFX => 4 RFSQX\n\
                                       176 ORE => 6 VJHF")?;
        let mut reactor = super::Reactor::new(&input);
        assert_eq!(reactor.ore_cost(1, "FUEL"), 180_697);
        Ok(())
    }

    #[test]
    fn test_14_ex5() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("171 ORE => 8 CNZTR\n\
                                       7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
                                       114 ORE => 4 BHXH\n\
                                       14 VRPVC => 6 BMBT\n\
                                       6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
                                       6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
                                       15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
                                       13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
                                       5 BMBT => 4 WPTQ\n\
                                       189 ORE => 9 KTJDG\n\
                                       1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
                                       12 VRPVC, 27 CNZTR => 2 XDBXC\n\
                                       15 KTJDG, 12 BHXH => 5 XCVML\n\
                                       3 BHXH, 2 VRPVC => 7 MZWV\n\
                                       121 ORE => 7 VRPVC\n\
                                       7 XCVML => 6 RJRHP\n\
                                       5 BHXH, 4 VRPVC => 5 LTCX")?;
        let mut reactor = super::Reactor::new(&input);
        assert_eq!(reactor.ore_cost(1, "FUEL"), 2_210_736);
        Ok(())
    }

    #[test]
    fn test_14() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_lines("input/day14.txt")?;
        assert_eq!(super::day14a(&input), 220_019);
        Ok(())
    }
}
