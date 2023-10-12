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
    let headers = vec![
        "Category",
        "School",
        "First Name",
        "Middle Name",
        "Last Name",
        "Coach Name",
        "Coach Email",
        "Coach Contact Number",
    ];

    for (i, header) in headers.iter().enumerate() {
        let column_letter = (b'A' + i as u8) as char;
        let cell_address = format!("{}1", column_letter);

        write_data(
            &mut book,
            sheet,
            &cell_address,
            Value::Text(header.to_string()),
        );

        let get_sheet = book.get_sheet_by_name_mut(sheet);

        match get_sheet {
            Ok(sheet) => {
                let style = sheet.get_style_mut(cell_address);
                style.get_font_mut().set_bold(true);
            }
            Err(err) => {
                eprintln!("Failed to set font style to bold: {err:?}");
            }
        }
    }

    for (i, row) in rows.iter().enumerate() {
        let fields = vec![
            "category",
            "school",
            "first_name",
            "middle_name",
            "last_name",
            "coach_name",
            "coach_email",
            "coach_contact_number",
        ];

        for (j, field) in fields.iter().enumerate() {
            let value: String = row.get(field);
            let column_letter = (b'A' + j as u8) as char;
            let cell_address = format!("{}{}", column_letter, i + 2);

            write_data(&mut book, sheet, &cell_address, Value::Text(value));

            let get_sheet = book.get_sheet_by_name_mut(sheet);

            match get_sheet {
                Ok(sheet) => {
                    sheet
                        .get_column_dimension_mut(column_letter.to_string().as_ref())
                        .set_auto_width(true);
                }
                Err(err) => {
                    eprintln!("Failed to set column auto width: {err:?}");
                }
            }
        }

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
