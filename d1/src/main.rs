use std::fs;

fn read_file() -> String {
    let contents = fs::read_to_string("day1_input")
    .expect("Should have been able to read the file");
    return contents;
}

fn part1(l1:&Vec<i32>,l2:&Vec<i32>) -> i32 {
    let mut sum = 0;
    for (i, &x) in l1.iter().enumerate() {
        sum += (l2[i]-l1[i]).abs();
    }
    sum
}

fn part2(l1:&Vec<i32>,l2:&Vec<i32>) -> i32 {
    let mut sum = 0;
    for (i, &x) in l1.iter().enumerate() {
        let count:i32 = l2.iter().filter(|&&z| z == l1[i]).count().try_into().unwrap();
        sum += l1[i]*count;
    }
    sum

}
fn main() {
    let contents = read_file();

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    contents.lines().for_each(|line| {
        let mut pair= line.split_whitespace();
        let x:i32 = pair.next().unwrap().parse().unwrap();
        let y:i32 = pair.next().unwrap().parse().unwrap();

        l1.push(x);
        l2.push(y);
    });
    l1.sort();
    l2.sort();
    

    let p1_ans  = part1(&l1, &l2);
    let p2_ans  = part2(&l1, &l2);

    println!("part 1 answer: {p1_ans}");
    println!("part 2 answer: {p2_ans}");
}