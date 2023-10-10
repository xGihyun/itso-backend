use axum::{extract::State, response::Result};
use sqlx::{PgPool, Row};
use umya_spreadsheet::*;

enum Value {
    Number(i32),
    Text(String),
}

pub async fn generate_excel(State(pool): State<PgPool>) -> Result<Vec<u8>> {
    let query = "SELECT * FROM participants";

    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch participants.");

    let mut book = new_file();
    let sheet = "Sheet1";

    write_data(
        &mut book,
        sheet,
        "A1",
        Value::Text("First Name".to_string()),
    );
    write_data(
        &mut book,
        sheet,
        "B1",
        Value::Text("Middle Name".to_string()),
    );
    write_data(&mut book, sheet, "C1", Value::Text("Last Name".to_string()));
    write_data(&mut book, sheet, "D1", Value::Text("School".to_string()));
    write_data(
        &mut book,
        sheet,
        "E1",
        Value::Text("Coach Name".to_string()),
    );
    write_data(&mut book, sheet, "F1", Value::Text("Category".to_string()));

    for (i, row) in rows.iter().enumerate() {
        let first_name: String = row.get("first_name");
        let middle_name: String = row.get("middle_name");
        let last_name: String = row.get("last_name");
        let school: String = row.get("school");
        let coach_name: String = row.get("coach_name");
        let category: String = row.get("category");

        write_data(
            &mut book,
            sheet,
            format!("A{}", i + 2).as_ref(),
            Value::Text(first_name),
        );
        write_data(
            &mut book,
            sheet,
            format!("B{}", i + 2).as_ref(),
            Value::Text(middle_name),
        );
        write_data(
            &mut book,
            sheet,
            format!("C{}", i + 2).as_ref(),
            Value::Text(last_name),
        );
        write_data(
            &mut book,
            sheet,
            format!("D{}", i + 2).as_ref(),
            Value::Text(school),
        );
        write_data(
            &mut book,
            sheet,
            format!("E{}", i + 2).as_ref(),
            Value::Text(coach_name),
        );
        write_data(
            &mut book,
            sheet,
            format!("F{}", i + 2).as_ref(),
            Value::Text(category),
        );

        println!("Successfully set values.");
    }

    let mut buffer: Vec<u8> = Vec::new();
    let writer = writer::xlsx::write_writer(&book, &mut buffer);

    match writer {
        Ok(()) => {
            println!("Successfully written spreadsheet.");
        }
        Err(err) => {
            eprintln!("Failed to write spreadsheet: {err:?}");
        }
    }

    Ok(buffer)
}

fn write_data(book: &mut Spreadsheet, sheet: &str, coordinate: &str, value: Value) {
    let get_sheet = book.get_sheet_by_name_mut(sheet);

    match get_sheet {
        Ok(sheet) => {
            let cell = sheet.get_cell_mut(coordinate);
            match value {
                Value::Text(s) => {
                    cell.set_value(s);
                }
                Value::Number(n) => {
                    cell.set_value_number(n);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to get worksheet: {err:?}");
        }
    }
}
