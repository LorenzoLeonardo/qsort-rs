use crate::qsort;

#[test]
fn test_numbers() {
    let mut arr = [5, 3, 8, 4, 2, 7, 1, 6];

    qsort::sort(&mut arr, |low, high| low <= high);

    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_strings() {
    let mut arr = ["king", "zebra", "banana", "juice", "apple"];

    qsort::sort(&mut arr, |low, high| low <= high);

    assert_eq!(arr, ["apple", "banana", "juice", "king", "zebra"]);
}

#[test]
fn test_custom_structs() {
    #[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
    struct Student {
        name: String,
        age: u32,
        gender: String,
    }

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

#[test]
fn random_order() {
    let mut numbers = Vec::new();
    for _ in 1..10000 {
        numbers.push(rand::random::<i32>());
    }

    qsort::sort(numbers.as_mut_slice(), |low, high| low <= high);

    for i in 0..numbers.len() - 1 {
        assert!(numbers[i] <= numbers[i + 1]);
    }
}

#[test]
fn custom_order() {
    let mut numbers = Vec::new();
    for _ in 1..10000 {
        numbers.push(rand::random::<i32>());
    }

    qsort::sort(numbers.as_mut_slice(), |low, high| low >= high);

    for i in 0..numbers.len() - 1 {
        assert!(numbers[i] >= numbers[i + 1]);
    }
}
