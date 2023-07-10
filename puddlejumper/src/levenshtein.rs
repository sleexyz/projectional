use std::cmp::min;


pub fn levenshtein(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut graph = vec![vec![0; b.len() + 1]; a.len() + 1];

    // initialize costs assuming nothing in common
    for i in 0..=a.len() {
        graph[i][0] = i;
    }
    for j in 0..=b.len() {
        graph[0][j] = j;
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };

            // min cost is min cost of nodes to the left, top, and top-left
            graph[i][j] = min(
                graph[i - 1][j] + 1, // deletion
                min(
                    graph[i][j - 1] + 1, // insertion
                    graph[i - 1][j - 1] + cost, // substitution
                ),
            );
        }
    }
    graph[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        let a = "kitten";
        let b = "sitting";
        assert_eq!(levenshtein(a, b), 3);
    }
}