pub struct Allergies {
    allergen_list: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs = 0b00000001,
    Peanuts = 0b00000010,
    Shellfish = 0b00000100,
    Strawberries  = 0b00001000,
    Tomatoes = 0b00010000,
    Chocolate = 0b00100000,
    Pollen = 0b01000000,
    Cats = 0b10000000,
}

impl Allergen {
    fn from_u32(value: u32) -> Allergen {
        match value {
            0b00000001 => Allergen::Eggs,
            0b00000010 => Allergen::Peanuts,
            0b00000100 => Allergen::Shellfish,
            0b00001000 => Allergen::Strawberries,
            0b00010000 => Allergen::Tomatoes,
            0b00100000 => Allergen::Chocolate,
            0b01000000 => Allergen::Pollen,
            0b10000000 => Allergen::Cats,
            _ => panic!("Unknown allergen value: {}", value)
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergen_list = Vec::new();

        for allergen_value in 0..8 {
            if score & (1 << allergen_value) != 0 {
                allergen_list.push(Allergen::from_u32(1 << allergen_value));
            }
        }

        Allergies {
            allergen_list : allergen_list,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergen_list.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies : Vec<Allergen> = vec![];

        for allergen in &self.allergen_list {

            allergies.push(allergen.clone());
        }

        allergies
    }
}
