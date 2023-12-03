use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bag_of_cubes.pest"]
pub struct BagOfCubesParser;