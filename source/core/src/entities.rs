#[derive(Debug)]
pub struct Prefixes {
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}

#[derive(Debug)]
pub struct RoamOutDTO {
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

#[derive(Debug)]
pub struct RoamOutDB {
    pub batch_id: i32,
    pub batch_date: String,
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
    pub prefix: String,
    pub country_id: Option<i32>,
    pub operator_id: Option<i32>,
}
