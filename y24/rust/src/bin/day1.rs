fn main() {
    println!("Day1:");
    let input = include_str!("../input/1");
    star1(input);
    star2(input);
}

pub fn star1(input: &str) {
    println!("Star 1:");
    let (mut list1, mut list2) = parse_lists(input);

    list1.sort();
    list2.sort();

    let result: i64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(e1, e2)| (e1 - e2).abs())
        .sum();
    println!("Result = {result}");
}

pub fn star2(input: &str) {
    println!("Star 2:");

    let (list1, list2) = parse_lists(input);

    let result: i64 = list1
        .iter()
        .map(|n1| n1 * (list2.iter().filter(|n2| *n2 == n1).count() as i64))
        .sum();
    println!("Result = {result}");
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut l| {
            (
                l.next().unwrap().parse::<i64>().unwrap(),
                l.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .unzip()
}
