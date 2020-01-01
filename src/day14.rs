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
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_14_ex1() -> Result<(), Box<dyn Error>> {
        let input: Vec<super::Reaction>
                  = util::parse_lines("10 ORE => 10 A\n\
                                       1 ORE => 1 B\n\
                                       7 A, 1 B => 1 C\n\
                                       7 A, 1 C => 1 D\n\
                                       7 A, 1 D => 1 E\n\
                                       7 A, 1 E => 1 FUEL")?;
        assert_eq!(input[0], super::Reaction { precursors: vec![super::Reagent { amount: 10, name: "ORE".to_owned() }],
                                               product: super::Reagent { amount: 10, name: "A".to_owned() } });
        Ok(())
    }
}
