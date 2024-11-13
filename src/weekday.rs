use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Weekday {
    Complimentary(Complimentary),
    Ordinary(Ordinary),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Complimentary {
    Vertu = 1,
    Génie = 2,
    Travail = 3,
    #[allow(non_camel_case_types)]
    lOpinion = 4,
    Récompenses = 5,
    Révolution = 6,
}

impl Display for Complimentary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Complimentary::Vertu => "La Fête de la Vertu",
                Complimentary::Génie => "La Fête du Génie",
                Complimentary::Travail => "La Fête du Travail",
                Complimentary::lOpinion => "La Fête de l'Opinion",
                Complimentary::Récompenses => "La Fête des Récompenses",
                Complimentary::Révolution => "La Fête de la Révolution",
            }
        )
    }
}

impl Complimentary {
    pub(crate) fn day_of_week(n: i32) -> Complimentary {
        match n {
            1 => Complimentary::Vertu,
            2 => Complimentary::Génie,
            3 => Complimentary::Travail,
            4 => Complimentary::lOpinion,
            5 => Complimentary::Récompenses,
            6 => Complimentary::Révolution,
            _ => unreachable!("invalid value passed to day of week"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ordinary {
    Primidi = 1,
    Duodi = 2,
    Tridi = 3,
    Quartidi = 4,
    Quintidi = 5,
    Sextidi = 6,
    Septidi = 7,
    Octidi = 8,
    Nonidi = 9,
    Décadi = 10,
}

impl Display for Ordinary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ordinary::Primidi => "Primidi",
                Ordinary::Duodi => "Duodi",
                Ordinary::Tridi => "Tridi",
                Ordinary::Quartidi => "Quartidi",
                Ordinary::Quintidi => "Quintidi",
                Ordinary::Sextidi => "Sextidi",
                Ordinary::Septidi => "Septidi",
                Ordinary::Octidi => "Octidi",
                Ordinary::Nonidi => "Nonidi",
                Ordinary::Décadi => "Décadi",
            }
        )
    }
}

impl Ordinary {
    pub(crate) fn day_of_week(n: i32) -> Ordinary {
        match n {
            1 => Ordinary::Primidi,
            2 => Ordinary::Duodi,
            3 => Ordinary::Tridi,
            4 => Ordinary::Quartidi,
            5 => Ordinary::Quintidi,
            6 => Ordinary::Sextidi,
            7 => Ordinary::Septidi,
            8 => Ordinary::Octidi,
            9 => Ordinary::Nonidi,
            10 => Ordinary::Décadi,
            _ => unreachable!("invalid value passed {:?}", n),
        }
    }
}
