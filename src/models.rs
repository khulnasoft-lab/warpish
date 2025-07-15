use diesel::prelude::*;
use crate::schema::commands;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = commands)]
pub struct Command {
    pub id: i32,
    pub timestamp: String,
    pub command: String,
    pub success: bool,
}

#[derive(Insertable)]
#[diesel(table_name = commands)]
pub struct NewCommand<'a> {
    pub timestamp: &'a str,
    pub command: &'a str,
    pub success: bool,
}
