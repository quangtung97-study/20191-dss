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

fn write_data(filename: &str, segments: &[Segment]) {
    let mut file = File::create(filename).unwrap();
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

    write_data("output", &result);
}

#[allow(dead_code)]
fn run_example2() {
    let mut set = {
        let a = point(0.0, 0.0);
        let b = point(0.75, 1.0);
        let c = point(1.0, 0.0);

        FuzzySet {
            segments: vec![segment(a, b), segment(b, c)],
        }
    };

    let func = {
        let a = point(0.0, 0.0);
        let b = point(0.25, 0.9);
        let c = point(1.0, 0.0);

        Function {
            segments: vec![segment(a, b), segment(b, c)],
        }
    };

    write_data("example2/data/input", &set.segments);
    write_data("example2/data/func", &func.segments);
    for _ in 0..2 {
        set = fuzzy::apply(&set, &func);
        show_segments(&set.segments);
        write_data("example2/data/output", &set.segments);
        println!("------------------------------------------");
    }
}

#[allow(dead_code)]
fn run_example2_3d() {
    let mut set = {
        let a = point(0.0, 0.0);
        let b = point(0.75, 1.0);
        let c = point(1.0, 0.0);

        FuzzySet {
            segments: vec![segment(a, b), segment(b, c)],
        }
    };

    let func = {
        let a = point(0.0, 0.0);
        let b = point(0.25, 0.9);
        let c = point(1.0, 0.0);

        Function {
            segments: vec![segment(a, b), segment(b, c)],
        }
    };

    write_data("example2/data3d/input", &set.segments);
    write_data("example2/data3d/func", &func.segments);
    for i in 0..20 {
        set = fuzzy::apply(&set, &func);
        show_segments(&set.segments);
        let path = format!("example2/data3d/output{}", i + 1);
        write_data(&path, &set.segments);
        println!("------------------------------------------");
    }
}

fn main() {
    run_example2_3d();
}
