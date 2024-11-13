use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Month {
    Vendémiaire,
    Brumaire,
    Frimaire,
    Nivôse,
    Pluviôse,
    Ventôse,
    Germinal,
    Floréal,
    Prairial,
    Messidor,
    Thermidor,
    Fructidor,
    Complémentaires,
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Month::Vendémiaire => "Vendémiaire",
                Month::Brumaire => "Brumaire",
                Month::Frimaire => "Frimaire",
                Month::Nivôse => "Nivôse",
                Month::Pluviôse => "Pluviôse",
                Month::Ventôse => "Ventôse",
                Month::Germinal => "Germinal",
                Month::Floréal => "Floréal",
                Month::Prairial => "Prairial",
                Month::Messidor => "Messidor",
                Month::Thermidor => "Thermidor",
                Month::Fructidor => "Fructidor",
                Month::Complémentaires => "Complémentaires",
            }
        )
    }
}

impl Month {
    pub fn value(&self) -> u8 {
        match self {
            Month::Vendémiaire => 1,
            Month::Brumaire => 2,
            Month::Frimaire => 3,
            Month::Nivôse => 4,
            Month::Pluviôse => 5,
            Month::Ventôse => 6,
            Month::Germinal => 7,
            Month::Floréal => 8,
            Month::Prairial => 9,
            Month::Messidor => 10,
            Month::Thermidor => 11,
            Month::Fructidor => 12,
            Month::Complémentaires => 13,
        }
    }

    pub fn next_month(&self) -> Month {
        match self {
            Month::Vendémiaire => Month::Brumaire,
            Month::Brumaire => Month::Frimaire,
            Month::Frimaire => Month::Nivôse,
            Month::Nivôse => Month::Pluviôse,
            Month::Pluviôse => Month::Ventôse,
            Month::Ventôse => Month::Germinal,
            Month::Germinal => Month::Floréal,
            Month::Floréal => Month::Prairial,
            Month::Prairial => Month::Messidor,
            Month::Messidor => Month::Thermidor,
            Month::Thermidor => Month::Fructidor,
            Month::Fructidor => Month::Complémentaires,
            Month::Complémentaires => Month::Vendémiaire,
        }
    }

    pub fn previous_month(&self) -> Month {
        match self {
            Month::Brumaire => Month::Vendémiaire,
            Month::Frimaire => Month::Brumaire,
            Month::Nivôse => Month::Frimaire,
            Month::Pluviôse => Month::Nivôse,
            Month::Ventôse => Month::Pluviôse,
            Month::Germinal => Month::Ventôse,
            Month::Floréal => Month::Germinal,
            Month::Prairial => Month::Floréal,
            Month::Messidor => Month::Prairial,
            Month::Thermidor => Month::Messidor,
            Month::Fructidor => Month::Thermidor,
            Month::Complémentaires => Month::Fructidor,
            Month::Vendémiaire => Month::Complémentaires,
        }
    }

    pub(crate) fn nth(n: u8) -> Month {
        match n {
            1 => Month::Vendémiaire,
            2 => Month::Brumaire,
            3 => Month::Frimaire,
            4 => Month::Nivôse,
            5 => Month::Pluviôse,
            6 => Month::Ventôse,
            7 => Month::Germinal,
            8 => Month::Floréal,
            9 => Month::Prairial,
            10 => Month::Messidor,
            11 => Month::Thermidor,
            12 => Month::Fructidor,
            13 => Month::Complémentaires,
            _ => unreachable!("invalid month: {}", n),
        }
    }
}
