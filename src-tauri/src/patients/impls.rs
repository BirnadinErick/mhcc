use async_trait::async_trait;
use super::models::*;
use crate::ports::PatientService;
use crate::adapters::PgAdapter;

#[async_trait]
impl StockService for PgAdapter {
	async fn add_patient(&self, add_patient: &AddPatient) -> u64 {
		let res = sqlx::query(
            "
INSERT INTO patients(patient_name, address, tpno, dob, nic)
VALUES (
    $1, $2, $3, $4, $5
 );
        ",
        )
        .bind(add_patient.patient_name.as_str())
        .bind(add_patient.address)
        .bind(add_patient.tpno)
        .bind(add_patient.dob)
        .bind(add_patient.nic)
        .execute(&self.pool)
        .await
        .expect("add_patient failed");

        res.rows_affected()
	}

    async fn get_patient(&self, offset: f64) -> Vec<GetPatient> {
		let patients: Vec<GetPatient> = sqlx::query_as!(
            GetPatient,
            r#"
SELECT
	patient_id,
	patient_name,
	dob,
	address,
	tpno,
	nic
FROM patients
WHERE
	patient_id >= 0 + (100 * $1)
		AND
	patient_id <= 100 + 100 * $1
ORDER BY patient_id,patient_name ASC
        "#,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .expect("Failed to fetch patients!");

        patients

	}

    async fn update_patient(&self, updated_patient: GetPatient) -> u64 {
		let query = format!(
            r#"
UPDATE patients
SET
    patient_name = '{}',
    dob = {},
    address = '{}',
    tpno = {}
    nic = {}
WHERE stock_id = {};
        "#,
            updated_patient.patient_name,
            updated_patient.dob,
            updated_patient.address,
            updated_patient.tpno,
            updated_patient.nic,
            updated_patient.patient_id
        );

        let res = sqlx::query(&query)
			.execute(&self.pool)
			.await
			.expect("couldn't update");

        res.rows_affected()

	}

    async fn search_patient_by_name(&self, query: String) -> Vec<GetPatient> {
		let patients: Vec<GetPatient> = sqlx::query_as!(
            GetPatient,
            r#"
SELECT
	patient_id,
	patient_name,
	dob,
	address,
	tpno,
	nic
FROM patients
WHERE patients.search_tokens @@ plainto_tsquery($1)
ORDER BY patient_id ASC, patient_name ASC;
        "#,
            query
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        patients
    }

    async fn search_patient_by_nic(&self, query: String) -> Vec<GetPatient> {
		todo!("search_patient_by_nic not yet implemented")
	}

}
