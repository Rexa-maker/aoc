fn main() {
    static INPUT: &str = include_str!("input");
    println!(
        "{} {}",
        total_score(INPUT, 100),
        total_score_for_calory(INPUT, 100, 500)
    );
}

#[derive(Copy, Clone)]
enum Feature {
    Capacity = 0,
    Durability,
    Flavor,
    Texture,
    COUNT,
}

impl Feature {
    fn iter() -> impl Iterator<Item = usize> {
        0..Feature::COUNT as usize // Ignore calories
    }
}

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    features: [i64; Feature::COUNT as usize],
    calories: i64,
}

impl Ingredient {
    fn parse_input(input: &str) -> Vec<Ingredient> {
        let mut ingredients = Vec::new();

        for line in input.lines() {
            // Line format: "INGREDIENT: capacity C, durability D, flavor F, texture T, calories C"
            let mut features = [0; Feature::COUNT as usize];
            let line = line.replace(",", "");
            let mut words = line.split_whitespace();
            features[Feature::Capacity as usize] = words.nth(2).unwrap().parse::<i64>().unwrap();
            features[Feature::Durability as usize] = words.nth(1).unwrap().parse::<i64>().unwrap();
            features[Feature::Flavor as usize] = words.nth(1).unwrap().parse::<i64>().unwrap();
            features[Feature::Texture as usize] = words.nth(1).unwrap().parse::<i64>().unwrap();
            let calories = words.nth(1).unwrap().parse::<i64>().unwrap();
            ingredients.push(Ingredient {
                features: features,
                calories: calories,
            });
        }

        ingredients
    }
}

struct IngredientCombination {
    ingredients: Vec<(i64, Ingredient)>,
}

impl IngredientCombination {
    fn score(&self) -> i64 {
        let mut features = [0; Feature::COUNT as usize];
        for (teaspoons, ingredient) in self.ingredients.iter() {
            for feature in Feature::iter() {
                features[feature] += ingredient.features[feature] * teaspoons;
            }
        }
        if features.iter().any(|&feature| feature <= 0) {
            return 0;
        }

        features.iter().product()
    }

    // Recursive combinations aggregator, not checking if the sum equals the total amount of teaspoons
    fn _teaspoons_to_combinations(teaspoons: u32, ingredients: u32) -> Vec<Vec<u32>> {
        let mut combinations = Vec::new();

        if ingredients == 1 {
            combinations = (0..=teaspoons).map(|x| vec![x]).collect::<Vec<Vec<u32>>>();
        } else {
            let sub_combinations =
                IngredientCombination::_teaspoons_to_combinations(teaspoons, ingredients - 1);
            for i in 0..=teaspoons {
                for mut sub_combination in sub_combinations.clone() {
                    sub_combination.push(i);
                    combinations.push(sub_combination);
                }
            }
        }

        combinations
    }

    /// From a given total amount of `teaspoons`, and a number `ingredients`,
    /// return a list of all the `combinations` of
    /// teaspoons per ingredients that add up to `teaspoons`.
    ///
    /// # Examples
    ///
    /// For a total amount of 4 teaspoons, and 2 ingredients, there are 5 combinations:
    ///  A  B  <- Ingredients
    /// (4, 0) |- Teaspoons
    /// (3, 1) v
    /// (2, 2)
    /// (1, 3)
    /// (0, 4)
    fn teaspoons_to_combinations(teaspoons: u32, ingredients: u32) -> Vec<Vec<u32>> {
        IngredientCombination::_teaspoons_to_combinations(teaspoons, ingredients)
            .iter()
            .filter(|combination| combination.iter().sum::<u32>() == teaspoons)
            .cloned()
            .collect::<Vec<Vec<u32>>>()
    }

    fn find_max_combination(teaspoons: u32, ingredients: Vec<Ingredient>) -> i64 {
        let mut max_score = 0;

        for u32_combination in
            IngredientCombination::teaspoons_to_combinations(teaspoons, ingredients.len() as u32)
        {
            let mut combination = Vec::new();
            for (teaspoons, ingredient) in u32_combination.iter().zip(ingredients.iter()) {
                combination.push((*teaspoons as i64, *ingredient));
            }
            max_score = max_score.max(
                IngredientCombination {
                    ingredients: combination,
                }
                .score(),
            );
        }

        max_score
    }
}

fn total_score(input: &str, teaspoons: u32) -> i64 {
    let ingredients = Ingredient::parse_input(input);
    IngredientCombination::find_max_combination(teaspoons, ingredients)
}

fn total_score_for_calory(input: &str, teaspoons: u32, calories: i64) -> i64 {
    let ingredients = Ingredient::parse_input(input);
    let mut max_score = 0;

    for u32_combination in
        IngredientCombination::teaspoons_to_combinations(teaspoons, ingredients.len() as u32)
    {
        let mut combination = Vec::new();
        for (teaspoons, ingredient) in u32_combination.iter().zip(ingredients.iter()) {
            combination.push((*teaspoons as i64, *ingredient));
        }
        let ingredient_combination = IngredientCombination {
            ingredients: combination,
        };
        if ingredient_combination
            .ingredients
            .iter()
            .map(|(teaspoons, ingredient)| teaspoons * ingredient.calories)
            .sum::<i64>()
            == calories
        {
            max_score = max_score.max(ingredient_combination.score());
        }
    }

    max_score
}

#[test]
fn examples() {
    assert_eq!(
        IngredientCombination::teaspoons_to_combinations(4, 1),
        [vec![4]]
    );
    let combinations = IngredientCombination::teaspoons_to_combinations(4, 2);
    let expected = vec![vec![4, 0], vec![3, 1], vec![2, 2], vec![1, 3], vec![0, 4]];
    assert_eq!(combinations, expected);
    let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                 Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(total_score(input, 100), 62842880);
    assert_eq!(total_score_for_calory(input, 100, 500), 57600000);
}
