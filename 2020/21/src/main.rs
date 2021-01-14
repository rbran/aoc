use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::InvalidData;
use std::str::FromStr;

type Err = Box<dyn std::error::Error>;

struct Product {
    ingredients: Vec<usize>,
    allergens: Vec<usize>,
}

struct Input {
    ingredients: Vec<String>,
    allergens: Vec<String>,
    products: Vec<Product>,
}

fn error(s: &str) -> Err {
    Box::new(Error::new(InvalidData, s.to_string()))
}

impl FromStr for Input {
    type Err = Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //store in HashMap so we can convert in a vector without repetitions
        let mut ingredients: HashMap<&str, usize> = HashMap::new();
        let mut allergens = HashMap::new();
        let mut products = Vec::new();
        //for each line populate products and also ingredients/allergens if is
        //the first time they appear
        for line in s.lines() {
            //in case the last line is empty
            if line.len() == 0 {
                break;
            }
            //product ingredients/allergens list for this product
            let mut product_ingre = Vec::new();
            let mut product_aller = Vec::new();

            let mut words = line.split(' ');
            //TODO: can I do that with for?
            loop {
                if let Some(word) = words.next() {
                    if word.len() == 0 {
                        return Err(error("Empty word"));
                    }
                    if word == "(contains" {
                        //start capturing allergens list
                        break;
                    }

                    //get the entry and create if necessary
                    let next_index = ingredients.len();
                    let entry = ingredients.entry(word).or_insert(next_index);
                    product_ingre.push(*entry);
                } else {
                    break;
                }
            }

            for word in words {
                if word.len() == 0 {
                    return Err(error("Empty word"));
                }
                let x: &[_] = &[',', ')'];
                let word = word.trim_end_matches(x);
                let next_index = allergens.len();
                let entry = allergens.entry(word).or_insert(next_index);
                product_aller.push(*entry);
            }

            products.push(Product {
                allergens: product_aller,
                ingredients: product_ingre,
            });
        }

        let conv_hashmap_vec = |map: &mut HashMap<&str, usize>| -> Vec<String> {
            let mut vec = vec!["".to_string(); map.len()];
            for (k, v) in map.drain() {
                let insert = vec.get_mut(v).unwrap();
                *insert = k.to_string();
            }
            vec
        };
        Ok(Input {
            ingredients: conv_hashmap_vec(&mut ingredients),
            allergens: conv_hashmap_vec(&mut allergens),
            products,
        })
    }
}

#[test]
fn test_parse_input() -> Result<(), Err> {
    const INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
    let ingredients =
        ["mxmxvkd", "kfcds", "sqjhc", "nhms", "trh", "fvjkl", "sbzzf"];
    let allergens = ["dairy", "fish", "soy"];
    let products_ingre: [&[_]; 4] =
        [&[0, 1, 2, 3], &[4, 5, 6, 0], &[2, 5], &[2, 0, 6]];
    let products_aller: [&[_]; 4] = [&[0, 1], &[0], &[2], &[1]];
    let input: Input = INPUT.parse()?;
    assert_eq!(input.ingredients.len(), ingredients.len());
    for i in 0..ingredients.len() {
        assert_eq!(input.ingredients[i], ingredients[i]);
    }
    assert_eq!(input.allergens.len(), allergens.len());
    for i in 0..allergens.len() {
        assert_eq!(input.allergens[i], allergens[i]);
    }
    assert_eq!(input.products.len(), products_ingre.len());
    for i in 0..products_ingre.len() {
        assert_eq!(
            input.products[i].ingredients.len(),
            products_ingre[i].len()
        );
        assert_eq!(input.products[i].allergens.len(), products_aller[i].len());
        for f in 0..products_ingre[i].len() {
            assert_eq!(input.products[i].ingredients[f], products_ingre[i][f]);
        }
        for f in 0..products_aller[i].len() {
            assert_eq!(input.products[i].allergens[f], products_aller[i][f]);
        }
    }
    Ok(())
}

struct Part1<'a> {
    input: &'a Input,
    aller_ingre: HashMap<usize, usize>,
    ingre_aller: HashMap<usize, Option<usize>>,
}

impl<'a> TryFrom<&'a Input> for Part1<'a> {
    type Error = Err;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        Ok(Part1 {
            input,
            aller_ingre: HashMap::new(),
            ingre_aller: HashMap::new(),
        })
    }
}

impl<'a> Part1<'a> {
    //TODO: sort the list and use binary search?
    fn solve(&mut self) -> Result<usize, Err> {
        let mut aller_ingre: HashMap<usize, Vec<usize>> = HashMap::new();
        for product in self.input.products.iter() {
            for aller in product.allergens.iter() {
                if let Some(entry) = aller_ingre.get_mut(aller) {
                    //"and" both arrays so we filter the ingredients in common
                    let new_entry = entry
                        .iter()
                        .filter(|x| product.ingredients.contains(x))
                        .copied()
                        .collect();
                    *entry = new_entry;
                } else {
                    aller_ingre.insert(*aller, product.ingredients.clone());
                }
            }
        }

        loop {
            //check if all allergens where discovered already
            if self.aller_ingre.len() == self.input.allergens.len() {
                break;
            }
            let mut changed = false;

            //find an invalid allergen, and classify known allergens
            for (aller, ingre) in aller_ingre.iter() {
                match ingre.len() {
                    0 => {
                        return Err(error("Allergenic can't be possible here"))
                    }
                    1 => {
                        changed = true;
                        self.aller_ingre.insert(*aller, ingre[0]);
                        self.ingre_aller.insert(ingre[0], Some(*aller));
                    }
                    _ => (),
                }
            }

            //each allergen can only be in one ingredient and vise-versa, so if
            //an allergen/ingredient pair is know, we can remove from the other
            //allergens list of possibilities, BTW ingredient can have no aller
            for (aller, ingre) in self.aller_ingre.iter() {
                aller_ingre.remove(aller);
                for (_, ingredients) in aller_ingre.iter_mut() {
                    let index = ingredients.iter().position(|x| x == ingre);
                    if let Some(x) = index {
                        changed = true;
                        ingredients.remove(x);
                    }
                }
            }

            if !changed {
                break;
            }
        }

        for (aller, ingre) in aller_ingre.iter() {
            print!("{}: ", self.input.allergens[*aller],);
            for ingre in ingre.iter() {
                print!("{},", self.input.ingredients[*ingre]);
            }
            println!();
        }

        //check ingredients that can't have any allergen
        let mut ingre_count = Vec::new(); //used to solve
        for (ingredient, _) in self.input.ingredients.iter().enumerate() {
            //check if they have define allergens
            if self.ingre_aller.contains_key(&ingredient) {
                continue;
            }
            //check if they are in list of possible allergens
            if aller_ingre
                .iter()
                .find(|(_, ingre)| ingre.contains(&ingredient))
                .is_none()
            {
                self.ingre_aller.insert(ingredient, None);
                ingre_count.push(ingredient);
            }
        }
        ingre_count.sort();
        let mut ret = 0;
        for product in self.input.products.iter() {
            for ingredient in product.ingredients.iter() {
                if let Ok(_) = ingre_count.binary_search(ingredient) {
                    ret += 1;
                }
            }
        }
        Ok(ret)
    }
}

struct Part2<'a> {
    part1: &'a Part1<'a>,
}

impl<'a> TryFrom<&'a Part1<'a>> for Part2<'a> {
    type Error = Err;
    fn try_from(part1: &'a Part1) -> Result<Self, Self::Error> {
        Ok(Part2 { part1 })
    }
}

impl<'a> Part2<'a> {
    fn solve(&mut self) -> Result<String, Err> {
        let mut aller: Vec<&String> = self
            .part1
            .aller_ingre
            .keys()
            .map(|x| self.part1.input.allergens.get(*x).unwrap())
            .collect();
        aller.sort();
        let ingre: Vec<String> = aller
            .iter()
            .map(|aller| {
                let aller_index = self
                    .part1
                    .input
                    .allergens
                    .iter()
                    .position(|x| x == *aller)
                    .unwrap();
                let ingre_index = self.part1.aller_ingre[&aller_index];
                self.part1.input.ingredients[ingre_index].clone()
            })
            .collect();
        Ok(ingre.join(","))
    }
}

fn main() -> Result<(), Err> {
    let input: String = fs::read_to_string(
        env::args().nth(1).unwrap_or("input.txt".to_string()),
    )?;
    let input: Input = input.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    println!("P1: {}", part1.solve()?);
    let mut part2 = Part2::try_from(&part1)?;
    println!("P2: {}", part2.solve()?);
    Ok(())
}

#[test]
fn test_example() -> Result<(), Err> {
    const INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
    let input = INPUT.parse()?;
    let mut part1 = Part1::try_from(&input)?;
    assert_eq!(part1.solve()?, 5);
    let mut part2 = Part2::try_from(&part1)?;
    assert_eq!(part2.solve()?, "mxmxvkd,sqjhc,fvjkl");
    Ok(())
}
