use num_bigint::BigInt;
use polars::prelude::{DataFrame, CsvReader, CsvWriter, SerReader, SerWriter, NamedFrom};
use polars::frame::UniqueKeepStrategy::First;
use polars::df;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use postgres::{Client, NoTls};

pub fn create_postgres_table(conn_string: String) {
    let mut client = Client::connect(&conn_string, NoTls).expect("Error establishing connection.");
    client.batch_execute("
    CREATE TABLE IF NOT EXISTS collatz (
        id       SERIAL PRIMARY KEY,
        Number   TEXT NOT NULL,
        Steps    TEXT NOT NULL
    )
    ").expect("Error creating psql table.");

}

pub fn update_psql(n: BigInt, steps: u32, conn_string: String) {
    let mut client = Client::connect(&conn_string, NoTls).expect("Error establishing connection.");

    client.execute(
        "INSERT INTO collatz (Number, Steps) VALUES ($1, $2)",
        &[&n.to_string(), &steps.to_string()],
    ).expect("Error updating psql table.");
}

pub fn update_flat_file(n: BigInt, step: u32) {

    let df_old: DataFrame = read_history();
    let new_data: DataFrame = make_new_frame(n, step);
    let mut df_new: DataFrame = stack_old_and_new(df_old, new_data);
    df_new = get_unique_df(df_new);

    write_new_csv(df_new);

    fn write_new_csv(mut df: DataFrame) {
        let mut file = std::fs::File::create("history.csv").unwrap();
        CsvWriter::new(&mut file).finish(&mut df).unwrap();
    }
    
    fn get_unique_df(df: DataFrame) -> DataFrame {
        df.unique(None, First).expect("DataFrame")
    }
    
    fn stack_old_and_new(df_old: DataFrame, df_new: DataFrame) -> DataFrame {
        df_old.vstack(&df_new).unwrap()
    }
    
    fn make_new_frame(n: BigInt, step: u32) -> DataFrame {
        df!("Number" => &[n.to_string()], "Steps" => &[step.to_string()]).expect("DataFrame")
    }
    
    fn read_history() -> DataFrame {
    
        if Path::new("history.csv").is_file() == false {
            let mut file: File = File::create("history.csv").expect("History file creation error");
            write!(file, "Number,Steps").expect("Issue writing headers to new history file.");
        }
        
        CsvReader::from_path("history.csv")
                .expect("File")
                .has_header(true)
                .infer_schema(Some(0)) // everything as utf8 so that BigInt cast will work
                .finish().expect("DataFrame")
    }
}