use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable)]
pub struct Flag {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
#[diesel(table_name=challenges)]
pub struct Challenge {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name=challenges)]
pub struct NewChallenge<'a> {
    pub name: &'a str,
}