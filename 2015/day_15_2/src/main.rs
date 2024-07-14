//! Advent of code 2015 day 15 part 2

use day_15_1::{Ingredient, IngredientEffect};
use std::io::BufRead;

trait Score {
    fn score(&self) -> Option<i64>;
}

impl Score for IngredientEffect {
    fn score(&self) -> Option<i64> {
        if self.calories == 500 {
            Some(
                i64::from(self.capacity.max(0))
                    * i64::from(self.durability.max(0))
                    * i64::from(self.flavor.max(0))
                    * i64::from(self.texture.max(0)),
            )
        } else {
            None
        }
    }
}

fn find_remaining_optimal_composition(
    current_effects: IngredientEffect,
    remaining_ingredients: &[IngredientEffect],
    remaining_amount: i32,
    optimal_amounts: &mut [i32],
) -> Option<i64> {
    assert!(!remaining_ingredients.is_empty());
    assert!(optimal_amounts.len() == remaining_ingredients.len());

    let ingredient = remaining_ingredients[0];
    if remaining_ingredients.len() == 1 {
        let effects = current_effects + ingredient * remaining_amount;
        optimal_amounts[0] = remaining_amount;
        effects.score()
    } else {
        let mut max_score = None;
        let mut best_amount = 0;
        let mut best_inner_amount = 0;
        for amount in 0..=remaining_amount {
            let effects = current_effects + ingredient * amount;
            let score = find_remaining_optimal_composition(
                effects,
                &remaining_ingredients[1..],
                remaining_amount - amount,
                &mut optimal_amounts[1..],
            );
            if score > max_score {
                max_score = score;
                best_amount = amount;
                best_inner_amount = optimal_amounts[1];
            }
        }
        optimal_amounts[0] = best_amount;
        optimal_amounts[1] = best_inner_amount;
        max_score
    }
}

struct Composition {
    amounts: Vec<i32>,
    score: i64,
}

fn find_optimal_composition(ingredients: &[Ingredient]) -> Option<Composition> {
    let mut amounts = vec![0; ingredients.len()];
    let score = find_remaining_optimal_composition(
        IngredientEffect::default(),
        &ingredients.iter().map(|i| i.effect).collect::<Vec<_>>(),
        100,
        &mut amounts,
    );
    score.map(|score| Composition { amounts, score })
}

fn main() {
    let file = std::fs::File::open("../day_15_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let ingredients: Vec<_> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| Ingredient::try_from(&*s).unwrap())
        .collect();

    let composition = find_optimal_composition(&ingredients).unwrap();
    println!("Optimal composition: {:?}", composition.amounts);
    println!("Optimal score: {}", composition.score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
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

        let cookie = butterscotch * 40 + cinnamon * 60;
        assert_eq!(cookie.score(), Some(57600000));
    }

    #[test]
    fn test_find_optimal_composition() {
        let ingredients = vec![
            Ingredient {
                name: "Butterscotch".to_string(),
                effect: IngredientEffect {
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8,
                },
            },
            Ingredient {
                name: "Cinnamon".to_string(),
                effect: IngredientEffect {
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3,
                },
            },
        ];
        let composition = find_optimal_composition(&ingredients).unwrap();
        assert_eq!(composition.score, 57600000);
        assert_eq!(composition.amounts, vec![40, 60]);
    }
}
