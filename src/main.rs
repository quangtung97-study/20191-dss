mod algorithm;
mod point;

use algorithm::{segment, Segment};
use point::point;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn input() -> Vec<Segment> {
    let mut result = Vec::new();

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<f32> = line
            .split_whitespace()
            .map(|s| s.parse::<f32>().unwrap())
            .collect();
        let a = point(numbers[0], numbers[1]);
        let b = point(numbers[2], numbers[3]);
        result.push(segment(a, b));
    }

    result
}

fn output(segments: &[Segment]) {
    let mut file = File::create("output").unwrap();
    for s in segments.iter() {
        writeln!(file, "{} {} {} {}", s.a.x, s.a.y, s.b.x, s.b.y).unwrap();
    }
}

fn main() {
    let segments = input();

    algorithm::show_segments(&segments);
    let result = algorithm::supremum(&segments);
    algorithm::show_segments(&result);

    output(&result);
}
