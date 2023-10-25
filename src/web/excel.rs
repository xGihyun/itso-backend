use axum::{extract::State, http, response::Result};
use sqlx::{PgPool, Row};

pub async fn generate_excel(
    State(pool): State<PgPool>,
) -> Result<(http::StatusCode, Vec<u8>), http::StatusCode> {
    let query = "SELECT * FROM participants";

    let get_participants = sqlx::query(query).fetch_all(&pool).await;

    match get_participants {
        Ok(rows) => {
            let mut csv_writer = csv::Writer::from_writer(Vec::new());

            let headers = [
                "Category",
                "School",
                "First Name",
                "Middle Name",
                "Last Name",
                "Coach Name",
                "Coach Email",
                "Coach Contact Number",
            ];

            csv_writer.write_record(&headers).unwrap();

            for row in rows.iter() {
                let fields = [
                    "category",
                    "school",
                    "first_name",
                    "middle_name",
                    "last_name",
                    "coach_name",
                    "coach_email",
                    "coach_contact_number",
                ];

                let mut csv_record = vec![];

                for field in fields.iter() {
                    let value: String = row.get(field);
                    csv_record.push(value);
                }

                csv_writer.write_record(&csv_record).unwrap();
            }

            let csv_data = csv_writer.into_inner().unwrap();

            Ok((http::StatusCode::OK, csv_data))
        }
        Err(err) => {
            eprintln!("Failed to get participants: {err:?}");
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
