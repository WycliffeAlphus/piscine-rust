#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _=> Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _=> Err(format!("Invalid RhFactor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.antigen.clone(), self.rh_factor.clone()).cmp(&(other.antigen.clone(), other.rh_factor.clone()))
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("Too short".to_string());
        }

        let (antigen_str, rh_str) = if s.starts_with("AB"){
            (&s[..2], &s[2..])
        } else {
            (&s[..1], &s[1..])
        };

        Ok(BloodType {
            antigen: Antigen::from_str(antigen_str)?,
            rh_factor: RhFactor::from_str(rh_str)?,
        })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_ok = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Negative, RhFactor::Positive) => false,
            _=> true,
        };

        let antigen_ok = match self.antigen {
            Antigen::O => matches!(other.antigen, Antigen::O),
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
        };
        rh_ok && antigen_ok
	}

	pub fn donors(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|b| self.can_receive_from(b)).collect()
	}

    pub fn recipients(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|b| b.can_receive_from(self)).collect()
    }

    fn all() -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;

        let mut types = vec![];

        for antigen in [O, A, B, AB] {
            for rh in [Negative, Positive] {
                types.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                });
            }
        }

        types
    }
}