use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub address: Option<String>,
    pub company_name: Option<String>,
    pub account: Option<String>,
    pub vat_id: Option<String>,
    pub country: String,
    pub currency: String,
    pub timezone: String,
    pub contact: Option<String>,
    pub email: Option<String>,
    pub homespage: Option<String>,
    pub mobile: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub budget: f32,
    pub time_budget: f32,
}
