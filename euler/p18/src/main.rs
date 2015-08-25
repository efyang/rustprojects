#![feature(vec_push_all)]
const TESTTRIANGLE: &'static [&'static [u64]] = &[
    &[3],
    &[7, 4],
    &[2, 4, 6],
    &[8, 5, 9, 3]];

const TRIANGLE: &'static [&'static [u64]] = &[
    &[75],
    &[95, 64],
    &[17, 47, 82],
    &[18, 35, 87, 10],
    &[20, 04, 82, 47, 65],
    &[19, 01, 23, 75, 03, 34],
    &[88, 02, 77, 73, 07, 63, 67],
    &[99, 65, 04, 28, 06, 16, 70, 92],
    &[41, 41, 26, 56, 83, 40, 80, 70, 33],
    &[41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    &[53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    &[70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    &[91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    &[63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    &[04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]];

fn max(x: &u64, y: &u64) -> u64 {
    if x > y {
        x.clone()
    }
    else {
        y.clone()
    }
}

fn max_vec(ns: &Vec<u64>) -> Vec<u64> {
    let mut new: Vec<u64> = Vec::new();
    for n in 0..(ns.len() - 1) {
        new.push(max(&ns[n], &ns[n + 1]));
    }
    new
}

fn add_vecs(xs: &Vec<u64>, ys: &Vec<u64>) -> Vec<u64> {
    xs.iter()
        .zip(ys.iter())
        .map(|x| x.0 + x.1)
        .collect()
}

fn max_path_sum(triangle: &[&[u64]]) -> u64 {
    let rows: usize = triangle.len();
    let last: &[u64] = triangle[rows - 1];
    let mut sums: Vec<u64> = Vec::new();
    sums.push_all(last);

    for row in (0..(triangle.len() - 1)).rev() {
        let currow = triangle[row];
        sums = max_vec(&sums);
        for x in 0..currow.len() {
            sums[x] += currow[x];
        } 
    }
    sums[0]
}

fn main() {
    println!("{}",max_path_sum(TRIANGLE));
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_vecs() {
        assert_eq!(vec![2u64, 5], super::add_vecs(&vec![1u64, 3], &vec![1u64, 2]));
    }

    #[test]
    fn max() {
        assert_eq!(20, super::max(&20u64, &13u64));
    }

    #[test]
    fn max_vec() {
        assert_eq!(vec![4u64, 3, 5], super::max_vec(&vec![4u64, 3, 2, 5]));
    }

    #[test]
    fn max_path_sum() {
        assert_eq!(23u64, super::max_path_sum(super::TESTTRIANGLE));
    }
}
