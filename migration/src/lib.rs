pub use sea_schema::migration::*;

mod m20220312_011700_create_user_table;
mod m20220312_011800_create_customer_table;
mod m20220312_011900_create_project_table;
mod m20220312_012000_create_time_record_table;
mod m20220312_012100_create_work_report_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220312_011700_create_user_table::Migration),
            Box::new(m20220312_011800_create_customer_table::Migration),
            Box::new(m20220312_011900_create_project_table::Migration),
            Box::new(m20220312_012000_create_time_record_table::Migration),
            Box::new(m20220312_012100_create_work_report_table::Migration),
        ]
    }
}
