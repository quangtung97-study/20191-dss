mod algorithm;
mod fuzzy;
mod point;

use algorithm::{segment, show_segments, supremum, Segment};
use fuzzy::{Function, FuzzySet};
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

#[allow(dead_code)]
fn run_supremum() {
    let segments = input();

    algorithm::show_segments(&segments);
    let result = supremum(&segments);
    algorithm::show_segments(&result);

    output(&result);
}

fn main() {
    let mut set = {
        let a = point(0.0, 0.0);
        let b = point(0.4, 1.0);
        let c = point(1.0, 0.2);

        FuzzySet {
            segments: vec![segment(a, b), segment(b, c)],
        }
    };

    let func = {
        let a = point(0.0, 0.0);
        let b = point(0.6, 1.0);
        let c = point(1.0, 0.0);

        Function {
            segments: vec![segment(a, b), segment(b, c)],
        }
    };

    for _ in 0..20 {
        set = fuzzy::apply(&set, &func);
        show_segments(&set.segments);
        output(&set.segments);
    }
}
