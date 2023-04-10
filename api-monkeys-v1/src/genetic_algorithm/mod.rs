use crate::requests::{AssessRequest, AssessResponse, TopRequest, TryRequest};
use std::ops::Range;
mod random;
use crate::genetic_algorithm::random::gen_random_char;
mod evolve_iter;
use self::client::RequestClient;
use self::random::{gen_random_float, gen_random_num_in_range, proportional_random};
use crate::genetic_algorithm::evolve_iter::EvolveIter;
use substring::Substring;

mod client;

pub async fn genetic_algorithm(try_request: &mut TryRequest) {
    convert_request(try_request);
    try_request.to_string();
    let request_client = RequestClient::new();
    get_length_dynamically(try_request, &request_client).await;
    let mut genomes: Vec<String> = create_genomes(try_request);
    let mut topScore = u32::MAX;
    for i in 0..try_request.limit {
        let assess_request = AssessRequest {
            id: try_request.id,
            genomes: genomes.clone(),
        };
        let assess_response = request_client.send_assess_req(assess_request).await.ok();
        let evolve_iter: EvolveIter;
        match assess_response {
            Some(resp) => evolve_iter = EvolveIter::new(&resp.scores),
            None => {
                print!("Error occured making assess request");
                return;
            }
        }

        if evolve_iter.top_score < topScore {
            topScore = evolve_iter.top_score;
            let index = evolve_iter
                .scores
                .iter()
                .position(|&score| score == evolve_iter.top_score)
                .unwrap();
            let top_request = TopRequest {
                id: try_request.id,
                iteration: i,
                score: topScore,
                genome: (*genomes[index]).to_string(),
            };
            request_client.send_top_req(top_request).await;
            if topScore == 0 {
                break;
            };
        }
        genomes = evolve_genomes(genomes, try_request, evolve_iter);
        println!("{:?}", genomes);
    }
}

async fn get_length_dynamically(try_request: &mut TryRequest, request_client: &RequestClient) {
    if try_request.length == 0 {
        //TO DO: do assesRequest to assess lambda
        let assess_request = AssessRequest {
            id: try_request.id,
            genomes: vec![String::new()],
        };
        let assess_response = request_client.send_assess_req(assess_request).await.ok();
        match assess_response {
            Some(resp) => try_request.length = resp.scores[0],
            None => {
                println!("Error making assess response with length specified as 0");
                return;
            }
        }
    }
}

fn evolve_genomes(
    mut genomes: Vec<String>,
    try_request: &mut TryRequest,
    evolve_iter: EvolveIter,
) -> Vec<String> {
    let end_range = try_request.monkeys / 2;
    let mut new_genomes: Vec<String> = Vec::new();
    for _j in 1..end_range {
        let parents: &mut Vec<String> = &mut evolve(&mut genomes, &evolve_iter, try_request);
        new_genomes.append(parents);
    }
    return new_genomes;
}

fn evolve(
    genomes: &mut Vec<String>,
    evolve_iter: &EvolveIter,
    try_request: &mut TryRequest,
) -> Vec<String> {
    let mut parent1 = genomes
        [proportional_random(&evolve_iter.weights, evolve_iter.weights_sum) as usize]
        .to_owned();
    let mut parent2 = genomes
        [proportional_random(&evolve_iter.weights, evolve_iter.weights_sum) as usize]
        .to_owned();
    mutate(&mut parent1, &mut parent2, &try_request);
    mutate_char(&mut parent1, &try_request);
    mutate_char(&mut parent2, &try_request);
    return vec![parent1, parent2];
}

fn mutate(parent1: &mut String, parent2: &mut String, try_request: &TryRequest) {
    if gen_random_float() < try_request.crossover {
        let x = gen_random_num_in_range(0, try_request.length);
        let end_index = x.try_into().unwrap();
        let mut child1_parent1 = parent1.substring(0, end_index).to_owned();
        let mut child1_parent2 = parent2.substring(0, end_index).to_owned();
        let child2_parent1 = parent1.substring(end_index, parent1.len());
        let child2_parent2 = parent2.substring(end_index, parent2.len());
        child1_parent1.push_str(child2_parent2);
        child1_parent2.push_str(child2_parent1);
        parent1.replace_range(0..parent1.len(), &child1_parent1);
        parent2.replace_range(0..parent2.len(), &child1_parent2);
    }
}

fn mutate_char(parent: &mut String, try_request: &TryRequest) {
    if gen_random_float() < try_request.mutation {
        let char_index = gen_random_num_in_range(0, try_request.length) as usize;
        let range: Range<usize>;
        if (char_index != 0) {
            range = char_index - 1..char_index;
        } else {
            range = 0..char_index + 1
        }
        parent.replace_range(range, &gen_random_char());
    }
}

fn convert_request(mut tryRequest: &mut TryRequest) {
    if tryRequest.monkeys % 2 != 0 {
        tryRequest.monkeys += 1;
    }
    tryRequest.crossover = tryRequest.crossover / 100.0;
    tryRequest.mutation = tryRequest.mutation / 100.0;
    if tryRequest.limit == 0 {
        tryRequest.limit = 1000;
    }
}

fn create_genomes(tryRequest: &mut TryRequest) -> Vec<String> {
    let mut genomes: Vec<String> = Vec::new();
    for _genome in 0..tryRequest.monkeys {
        let mut s = String::from("");
        for _i in 0..tryRequest.length {
            let character = gen_random_char();
            s.push_str(&character);
        }
        println!("{}\n", s);
        genomes.push(s);
    }
    return genomes;
}
