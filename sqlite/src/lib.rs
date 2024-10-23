use csv::ReaderBuilder; // for loading from csv
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; // for loading csv // for capturing errors from loading

// Create a table with the new schema
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            music_id INTEGER PRIMARY KEY,
            track_name TEXT NOT NULL,
            artist TEXT NOT NULL,
            in_spotify_charts INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(())
}

// Read data from the table
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        let music_id: i64 = row.get(0)?;
        let track_name: String = row.get(1)?;
        let artist: String = row.get(2)?;
        let in_spotify_charts: i64 = row.get(3)?;
        Ok((music_id, track_name, artist, in_spotify_charts))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (music_id, track_name, artist, in_spotify_charts) = row?;
        println!(
            "Music ID: {}, Track Name: {}, Artist: {}, In Spotify Charts: {}",
            music_id, track_name, artist, in_spotify_charts
        );
    }

    Ok(())
}

// Delete the table
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Load data from a CSV file into the table with the new schema
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (music_id, track_name, artist, in_spotify_charts) VALUES (?, ?, ?, ?)",
        table_name
    );

    // Loop that expects the specific schema
    for result in rdr.records() {
        let record = result?;
        let music_id: i64 = record[0].parse().expect("Failed to parse music_id");
        let track_name: &str = &record[1]; //.parse().expect("Failed to parse track_name");
        let artist: &str = &record[2]; //.parse().expect("Failed to parse artist");
        let in_spotify_charts: i64 = record[3].parse().expect("Failed to parse in_spotify_charts");

        conn.execute(&insert_query, params![music_id, track_name, artist, in_spotify_charts])?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

// Update table records
pub fn update_table(
    conn: &Connection,
    table_name: &str,
    set_clause: &str,
    condition: &str,
) -> Result<()> {
    // Construct the SQL UPDATE query using the provided table name, set clause, and condition
    let update_query = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name, set_clause, condition
    );
    
    // Execute the update query
    let affected_rows = conn.execute(&update_query, [])?;
    
    // Output the number of rows updated
    println!(
        "Successfully updated {} row(s) in table '{}'.",
        affected_rows, table_name
    );
    
    Ok(())
}
