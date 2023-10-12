use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub struct Grade {
    grade: u32
}

impl Grade {
    pub fn new_from_decimal(decimal: u32) -> Self {
        Grade { grade: decimal }
    }

    pub fn new_from_ternary(ternary: &str) -> Self {
        Grade {
            grade: ternary.chars().rev().enumerate().fold(0, |acc, (i, c)| {
                acc + c.to_digit(10).unwrap() * 3u32.pow(i as u32)
            })
        }
    }

    pub fn get_grade(&self) -> u32 {
        self.grade
    }

    pub fn decimal_to_ternary(&self) -> String {
        let mut decimal = self.grade;
        let mut ternary = String::new();

        while decimal > 0 {
            ternary.push_str(&(decimal % 3).to_string());
            decimal /= 3;
        }

        ternary.chars().rev().collect()
    }

    pub fn get_color_boxes(&self) -> String {
        let mut color_boxes = String::new();
        let mut i: u32 = self.grade;
        for _ in 0..5 {
            match i % 3 {
                0 => color_boxes.push_str("⬛"),
                1 => color_boxes.push_str("🟨"),
                2 => color_boxes.push_str("🟩"),
                _ => color_boxes.push_str("?")
            }
            i = i / 3;
        }
        color_boxes.chars().rev().collect()
    }
}

impl PartialEq for &Grade {
    fn eq(&self, other: &Self) -> bool {
        self.grade == other.grade
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_color_boxes())
    }
}
