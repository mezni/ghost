use core::db::DBManager;
use core::entities::RoamOutDB;
use core::errors::AppError;
use tokio_postgres::Client;

const INSERT_IMSI_QUERY: &str = "
    INSERT INTO dim_imsi (roam_type_id, imsi)
    SELECT 
        (SELECT id FROM dim_roam_type WHERE roam_type = 'OUT') AS roam_type_id,
        imsi
    FROM 
        stg_roam_out stg
    EXCEPT
    SELECT roam_type_id, imsi
    FROM dim_imsi
";

const INSERT_MSISDN_QUERY: &str = "
    INSERT INTO dim_msisdn (roam_type_id, msisdn)
    SELECT 
        (SELECT id FROM dim_roam_type WHERE roam_type = 'OUT') AS roam_type_id,
        msisdn
    FROM 
        stg_roam_out stg
    EXCEPT
    SELECT roam_type_id, msisdn
    FROM dim_msisdn
";

const INSERT_ROAM_OUT_QUERY: &str = "INSERT INTO fct_roam_out (date_id , batch_id , country_id , operator_id , imsi_id , msisdn_id )
SELECT tim.id , stg.batch_id , stg.country_id , stg.operator_id , ims.id , msi.id
FROM 
    stg_roam_out stg RIGHT JOIN dim_imsi ims ON stg.imsi = ims.imsi
    JOIN dim_msisdn msi ON stg.msisdn = msi.msisdn
    JOIN dim_time tim ON stg.batch_date = tim.date_text
    ";

// Function to insert IMSI records into the dim_imsi table
pub async fn insert_imsi_records(client: &Client) -> Result<(), AppError> {
    client
        .execute(INSERT_IMSI_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;
    Ok(())
}

// Function to insert MSISDN records into the dim_msisdn table
pub async fn insert_msisdn_records(client: &Client) -> Result<(), AppError> {
    client
        .execute(INSERT_MSISDN_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;
    Ok(())
}

pub async fn insert_fct_roam_out_records(client: &Client) -> Result<(), AppError> {
    client
        .execute(INSERT_ROAM_OUT_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;
    Ok(())
}

// Main function to insert roam out records into the staging table and then update the dimension tables
pub async fn insert_stg_roam_out_records(
    client: &Client,
    db_records: Vec<RoamOutDB>,
) -> Result<(), AppError> {
    // First insert into the staging table
    for record in db_records {
        client
            .execute(
                "INSERT INTO stg_roam_out (batch_id, batch_date, imsi, msisdn, vlr_number, country_id, operator_id) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[
                    &record.batch_id,
                    &record.batch_date,
                    &record.imsi,
                    &record.msisdn,
                    &record.vlr_number,
                    &record.country_id,
                    &record.operator_id,
                ],
            )
            .await
            .map_err(AppError::DatabaseError)?;
    }

    Ok(())
}
