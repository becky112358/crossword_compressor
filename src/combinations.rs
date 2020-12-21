
use factorial::Factorial;

pub fn get_combinations(n_words: usize) -> Vec<Vec<usize>> {
    let mut orderings = Vec::with_capacity(n_words.factorial());

    for i in 0..n_words {
        let mut ordering = vec![n_words; n_words];

        ordering[i] = 0;

        set_remaining_options(n_words, 1, ordering, &mut orderings);
    }

    println!("{:#?}", orderings);

    return orderings;
}

fn set_remaining_options(n: usize, current_index: usize, ordering: Vec<usize>, orderings: &mut Vec<Vec<usize>>) {
    for i in 0..n {
        if ordering[i] == n {
            let mut ordering = ordering.clone();
            ordering[i] = current_index;
            if current_index < n - 1 {
                set_remaining_options(n, current_index+1, ordering, orderings);
            } else {
                orderings.push(ordering);
            }
        }
    }
}


#[cfg(test)]
#[path = "./tests_combinations.rs"]
mod tests_combinations;


