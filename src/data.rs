use serde::{Serialize, Serializer};
use std::convert::TryInto;

/// Struct to imitate the Django Model in https://github.com/voidfiles/python-serialization-benchmark/blob/master/subjects/rf.py
#[derive(Serialize, Debug)]
pub struct SubRF {
    w: i32,
    #[serde(serialize_with = "get_x")]
    x: i32,
    y: String,
    z: i32,
}

/// implement 'get_x' method from Django Rest Framework serializer
/// https://github.com/voidfiles/python-serialization-benchmark/blob/master/subjects/rf.py#L18
fn get_x<S>(x: &i32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i32(x + 10)
}

impl SubRF {
    /// constructor to imitate https://github.com/voidfiles/python-serialization-benchmark/blob/master/data.py
    pub fn new(multiplier: Option<i32>) -> Self {
        match multiplier {
            Some(m) => SubRF {
                w: 1000 * m,
                x: 20 * m,
                y: "hello".repeat(m.try_into().unwrap()),
                z: 10 * m,
            },
            None => SubRF {
                w: 100,
                x: 20,
                y: String::from("hello"),
                z: 10,
            },
        }
    }
}

/// Struct to imitate the Django Model in https://github.com/voidfiles/python-serialization-benchmark/blob/master/subjects/rf.py
#[derive(Serialize)]
pub struct ComplexRF {
    foo: &'static str,
    #[serde(serialize_with = "get_bar")]
    bar: u8,
    sub: SubRF,
    subs: [SubRF; 10],
}

impl ComplexRF {
    /// constructor to imitate https://github.com/voidfiles/python-serialization-benchmark/blob/master/data.py
    pub fn new() -> Self {
        ComplexRF {
            foo: "bar",
            bar: Default::default(),
            sub: SubRF::new(None),
            subs: (0..10)
                .into_iter()
                .map(|n| SubRF::new(Some(n)))
                .collect::<Vec<SubRF>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn bar() -> u8 {
        5
    }
}

/// implement 'get_bar' to imitate the 'ComplexRF.bar()' method from Django Rest Framework serializer
/// https://github.com/voidfiles/python-serialization-benchmark/blob/master/subjects/rf.py
fn get_bar<S>(_: &u8, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u8(5)
}
