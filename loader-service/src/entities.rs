#[derive(Debug)]
pub struct RoamOutDAO {
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
    pub carrier_name: String,
    pub country_name: String,
}
