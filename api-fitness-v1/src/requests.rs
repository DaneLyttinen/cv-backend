use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TryRequest{
    pub id: u32,
    pub monkeys: u32,
    pub length: u32,
    pub crossover: f64,
    pub mutation: f64,
    pub limit: u32,
}

impl fmt::Display for TryRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{},{},{}", self.id, self.monkeys, self.length, self.crossover, self.mutation, self.limit)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopRequest{
    pub id: u32,
    pub iteration: u32,
    pub score: u32,
    pub genome: String,
}

impl fmt::Display for TopRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{}", self.id, self.iteration, self.score, self.genome)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssessRequest{
    pub id: u32,
    pub genomes: Vec<String> 
}

impl fmt::Display for AssessRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:?}", self.id, self.genomes)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AssessResponse{
    pub id: u32,
    pub scores: Vec<u32> 
}

impl fmt::Display for AssessResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:?}", self.id, self.scores)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TargetRequest{
    pub id: u32,
    pub parallel: bool,
    pub target: String
}

impl fmt::Display for TargetRequest{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.id, self.parallel, self.target)
    }
}