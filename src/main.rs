#[allow(unused_imports)]
use std::io::Read;

use crossterm::style::Stylize;
use rusqlite::{Connection, Row};

macro_rules! sql {
    ($conn:expr, { $($query:tt)* }) => {{
        let sql_text = stringify!($($query)*)
            .replace('\n', " ")
            .replace('\t', " ");
        for stmt in sql_text.split(';') {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }

            if stmt.to_uppercase().starts_with("SELECT") {
                let mut statement = $conn.prepare(stmt).unwrap();


                let column_count = statement.column_count();
                let rows = statement.query_map([], move |row: &Row| {
                    let mut values = Vec::new();
                    for i in 0..column_count {
                        match row.get::<_, rusqlite::types::Value>(i) {
                            Ok(value) => values.push(format!("{:?}", value)),
                            Err(e) => eprintln!("{}", format!("Error retrieving column {}: {}", i, e).red()),
                        }
                    }
                    Ok(values.join(" | "))
                }).unwrap();

                println!("{}", format!("{}: ", stmt).bold().yellow());
                for result in rows {
                    match result {
                        Ok(value) => println!("{}", value.blue()),
                        Err(e) => eprintln!("{}", format!("Error retrieving row: {}", e).red()),
                    }
                }
            } else {
                println!("{}", format!("{}: ", stmt).bold().yellow());
                match $conn.execute(stmt, []) {
                    Ok(count) => println!("{}", format!("{} rows affected", count).green()),
                    Err(e) => eprintln!("{}", format!("Error executing statement: {}", e).red()),
                }
            }
        }
    }};
}


fn main() {
    let conn = Connection::open_in_memory().unwrap();
    sql!(conn, {
        CREATE TABLE cryptocoin (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            symbol TEXT NOT NULL,
            price REAL NOT NULL
        );

        INSERT INTO cryptocoin (name, symbol, price) VALUES
            ("Bitcoin", "BTC", 45000.0),
            ("Ethereum", "ETH", 3000.0),
            ("Ripple", "XRP", 0.5),
            ("Litecoin", "LTC", 150.0);
        
        SELECT * FROM cryptocoin;
        UPDATE cryptocoin SET price = 48000.0 WHERE symbol = "BTC";
        SELECT * FROM cryptocoin;
    });

    //std::io::stdin().read(&mut [0]).unwrap();
}
