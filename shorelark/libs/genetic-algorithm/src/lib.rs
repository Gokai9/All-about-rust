use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::{Rng, RngCore};
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    fn new(&self) -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        //population.choose_weighted(rng, |individual| individual.fitness()).expect("got an empty space")

        let total_fitness: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();

        loop {
            let indiv = population.choose(rng).expect("Value is empty");
            let indiv_random = indiv.fitness() / total_fitness;

            if rng.gen_bool(indiv_random as f64) {
                return indiv;
            }
        }
    }
}

impl GeneticAlgorithm {
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let a = self.selection_method.select(rng, population);
                let b = self.selection_method.select(rng, population);
                //todo selection
                //todo crossover
                //todo mutation
            })
            .collect()
    }
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[test]
fn test() {
    let method = RouletteWheelSelection::new();
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let population = vec![
        TestIndividual::new(2.0),
        TestIndividual::new(1.0),
        TestIndividual::new(4.0),
        TestIndividual::new(3.0),
    ];

    let mut actual_histogram = BTreeMap::new();

    for _ in 0..1000 {
        let fitness = method.select(&mut rng, &population).fitness() as i32;

        *actual_histogram.entry(fitness).or_insert(0) += 1;
    }

    let expected_histogram = BTreeMap::from_iter(vec![
        // (fitness, how many times this fitness has been chosen)
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
    ]);

    assert_eq!(actual_histogram, expected_histogram);
}
