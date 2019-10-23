use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: i32,
}

pub fn parse() {
    println!("24 days of Rust - csv (day 3)");
    let dollar_films = vec![
        ("title", "bad_guy", "pub_year"),
        ("A Fistful of Dollars", "Rojo", "1964"),
        ("For a Few Dollars More", "El Indio", "1965"),
        ("The Good, the Bad and the Ugly", "Tuco", "1966"),
    ];
    let path = "westerns.csv";
    let mut writer = Writer::from_path(path).unwrap();
    for row in dollar_films {
        writer.serialize(row).expect("CSV writer error");
    }
    let movie = Movie {
        title: "Hang 'Em High".to_string(),
        bad_guy: "Wilson".to_string(),
        pub_year: 1968,
    };
    writer.serialize(movie).expect("CSV writer error");
    writer.flush().expect("Flush error");

    let mut reader = Reader::from_path(path).unwrap();
    for row in reader.deserialize::<Movie>() {
        let movie = row.unwrap();
        println!(
            "{} was a bad guy in '{}' in {}",
            movie.bad_guy, movie.title, movie.pub_year
        );
    }
}
