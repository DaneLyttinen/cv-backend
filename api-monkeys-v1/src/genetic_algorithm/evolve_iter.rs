pub struct EvolveIter{
    pub scores: Vec<u32>,
    pub top_score: u32,
    pub max_score: u32,
    pub weights: Vec<u32>,
    pub weights_sum: u32
}

impl EvolveIter {
    pub fn new(scores: &Vec<u32>) -> EvolveIter{
        let mut top_score = scores.iter().min().unwrap();
        let max_score = scores.iter().max().unwrap();
        let weights: Vec<u32> = scores.iter().map(|score| max_score - score + 1).collect();
        let weights_sum: u32 = scores.iter().sum();
        EvolveIter{scores: scores.to_vec(),top_score:*top_score, max_score: *max_score, weights: weights, weights_sum: weights_sum}
    }
}