use rusql_alchemy::prelude::*;

#[derive(Model, Clone, FromRow, Debug)]
pub struct Pom {
    #[model(primary_key = true, auto = true)]
    pub id: Integer,

    #[model(unique = true, null = false, size = 10)]
    pub name: String,

    #[model(default = 25)]
    pub duration_minutes: Integer,

    #[model(default = 5)]
    pub break_duration_minutes: Integer,
}
