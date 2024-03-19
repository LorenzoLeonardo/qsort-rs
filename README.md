# qsort-rs

A quick sort algorithm that accepts any type and non-recursive approach.

[![Latest Version](https://img.shields.io/crates/v/qsort-rs.svg)](https://crates.io/crates/qsort-rs)
[![License](https://img.shields.io/github/license/LorenzoLeonardo/qsort-rs.svg)](LICENSE)
[![Documentation](https://docs.rs/qsort-rs/badge.svg)](https://docs.rs/qsort-rs)
[![Build Status](https://github.com/LorenzoLeonardo/qsort-rs/workflows/Rust/badge.svg)](https://github.com/LorenzoLeonardo/qsort-rs/actions)

## How to use this function

```rust
use qsort_rs::qsort;

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
struct Student {
    name: String,
    age: u32,
    gender: String,
}

fn main() {
    let mut arr = [
        Student {
            name: String::from("Zed"),
            age: 3,
            gender: String::from("Male"),
        },
        Student {
            name: String::from("Aubrey"),
            age: 13,
            gender: String::from("Female"),
        },
        Student {
            name: String::from("Jaime"),
            age: 6,
            gender: String::from("Male"),
        },
        Student {
            name: String::from("Irene"),
            age: 8,
            gender: String::from("Female"),
        },
        Student {
            name: String::from("Ren"),
            age: 9,
            gender: String::from("Male"),
        },
    ];

    qsort::sort(&mut arr, |low, high| low.name <= high.name);

    assert_eq!(
        arr,
        [
            Student {
                name: "Aubrey".to_string(),
                age: 13,
                gender: "Female".to_string()
            },
            Student {
                name: "Irene".to_string(),
                age: 8,
                gender: "Female".to_string()
            },
            Student {
                name: "Jaime".to_string(),
                age: 6,
                gender: "Male".to_string()
            },
            Student {
                name: "Ren".to_string(),
                age: 9,
                gender: "Male".to_string()
            },
            Student {
                name: "Zed".to_string(),
                age: 3,
                gender: "Male".to_string()
            }
        ]
    )
}
```
