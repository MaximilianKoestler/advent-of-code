#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IngredientEffect {
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

impl std::ops::Add for IngredientEffect {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        IngredientEffect {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
}

impl std::ops::Mul<i32> for IngredientEffect {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        IngredientEffect {
            capacity: self.capacity * other,
            durability: self.durability * other,
            flavor: self.flavor * other,
            texture: self.texture * other,
            calories: self.calories * other,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub effect: IngredientEffect,
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::alpha1,
        combinator::map,
        sequence::{preceded, tuple},
        IResult,
    };

    use super::{Ingredient, IngredientEffect};

    fn name(input: &str) -> IResult<&str, String> {
        map(alpha1, |name: &str| name.to_string())(input)
    }

    fn capacity(input: &str) -> IResult<&str, i32> {
        preceded(tag("capacity "), nom::character::complete::i32)(input)
    }

    fn durability(input: &str) -> IResult<&str, i32> {
        preceded(tag("durability "), nom::character::complete::i32)(input)
    }

    fn flavor(input: &str) -> IResult<&str, i32> {
        preceded(tag("flavor "), nom::character::complete::i32)(input)
    }

    fn texture(input: &str) -> IResult<&str, i32> {
        preceded(tag("texture "), nom::character::complete::i32)(input)
    }

    fn calories(input: &str) -> IResult<&str, i32> {
        preceded(tag("calories "), nom::character::complete::i32)(input)
    }

    pub fn ingredient(input: &str) -> IResult<&str, Ingredient> {
        let (input, (name, _, capacity, _, durability, _, flavor, _, texture, _, calories)) =
            tuple((
                name,
                tag(": "),
                capacity,
                tag(", "),
                durability,
                tag(", "),
                flavor,
                tag(", "),
                texture,
                tag(", "),
                calories,
            ))(input)?;

        Ok((
            input,
            Ingredient {
                name,
                effect: IngredientEffect {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            },
        ))
    }
}

impl<'a> TryFrom<&'a str> for Ingredient {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::ingredient(input).map(|(_, c)| c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingredient() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
        assert_eq!(
            Ingredient::try_from(input),
            Ok(Ingredient {
                name: "Butterscotch".to_string(),
                effect: IngredientEffect {
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8,
                }
            })
        );

        let input = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(
            Ingredient::try_from(input),
            Ok(Ingredient {
                name: "Cinnamon".to_string(),
                effect: IngredientEffect {
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3,
                }
            })
        );
    }

    #[test]
    fn test_arithmetic() {
        let butterscotch = IngredientEffect {
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        };
        let cinnamon = IngredientEffect {
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        };
        let cookie = butterscotch * 44 + cinnamon * 56;
        assert_eq!(
            cookie,
            IngredientEffect {
                capacity: 68,
                durability: 80,
                flavor: 152,
                texture: 76,
                calories: 520,
            }
        );
    }
}
