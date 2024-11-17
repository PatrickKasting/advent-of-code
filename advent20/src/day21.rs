use ahash::AHashSet;
use itertools::Itertools;

type Line<'input> = (AHashSet<Ingredient<'input>>, AHashSet<Allergen<'input>>);
type Ingredient<'input> = &'input str;
type Allergen<'input> = &'input str;

pub fn first_answer(input: &str) -> String {
    let list = list(input);
    let ingredients_that_can_contain_any_allergen =
        ingredients_that_can_contain_any_allergen(&list);
    all_ingredients(&list)
        .filter(|ingredient| !ingredients_that_can_contain_any_allergen.contains(ingredient))
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let list = list(input);
    todo!()
}

fn ingredients_that_can_contain_any_allergen<'input>(
    list: &[Line<'input>],
) -> AHashSet<Ingredient<'input>> {
    all_allergens(list)
        .into_iter()
        .flat_map(|allergen| ingredients_that_can_contain_particular_allergen(list, allergen))
        .collect()
}

fn all_allergens<'input>(list: &[Line<'input>]) -> AHashSet<Allergen<'input>> {
    list.iter()
        .flat_map(|(_, allergens)| allergens.iter().copied())
        .collect()
}

fn ingredients_that_can_contain_particular_allergen<'input>(
    list: &[Line<'input>],
    allergen: Allergen<'input>,
) -> AHashSet<Ingredient<'input>> {
    let mut foods_containing_allergen = list
        .iter()
        .filter(|(_, allergens)| allergens.contains(allergen))
        .map(|(ingredients, _)| ingredients);
    let mut ingredients_that_can_contain = foods_containing_allergen
        .next()
        .cloned()
        .unwrap_or_default();
    for ingredients in foods_containing_allergen {
        ingredients_that_can_contain = &ingredients_that_can_contain & ingredients;
    }
    ingredients_that_can_contain
}

fn all_ingredients<'input, 'list: 'input>(
    list: &'list [Line<'input>],
) -> impl Iterator<Item = Ingredient<'input>> + 'input {
    list.iter()
        .flat_map(|(ingredients, _)| ingredients.iter().copied())
}

fn list(input: &str) -> Vec<Line> {
    input.lines().map(line).collect_vec()
}

fn line(line: &str) -> Line {
    let (ingredients, allergens) = line
        .split_once("(contains ")
        .expect("ingredients and allergens should be separated");
    let ingredients = ingredients.split_whitespace().collect();
    let allergens = allergens
        .strip_suffix(')')
        .expect("every line should end in a ')'")
        .split(", ")
        .collect();
    (ingredients, allergens)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 21;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2517);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 12);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 228);
    }
}
