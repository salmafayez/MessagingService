use std::sync::{Arc, Mutex};
use std::thread;
use oracle::{Connection, Row};
use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    pub name: String,
    pub phone: String,
    pub segment: String,
    pub balance: i32
}

pub fn  getCustomerPage(pageNumber: &i32, c: &Connection) -> Vec<Customer> {
    let sql = format!("{}{}{}", "select * FROM customer ORDER BY phone OFFSET ", pageNumber*100000, " ROWS FETCH NEXT 100000 ROWS ONLY");
    let rows = c.query(&sql, &[]).expect("can't execute query");
    let mut customers : Vec<Customer> = vec![];
    for row_result in  rows {
        let row = row_result.expect("No result returned");
        let customer = buildCustomer(&row);
        customers.push(customer);
    }
    customers
}

pub fn getCustomersCount() -> i32 {
    let conn = getConnection();

    let mut count = 0;
    let sql = "SELECT COUNT(*) FROM CUSTOMER";

    let rows = conn.query(&sql, &[]).expect("can't execute query");
    for row_result in rows {
        let row = row_result.expect("No result returned");
        count = row.get(0).expect("there is no column ");
    }
    count
}

pub fn getConnection() -> Connection{
    Connection::connect(
        "poc",
        "poc",
        "//localhost:1521/orclpdb1").expect("Error connection")
}

pub fn getNumberOfPages(usersCount : &i32) -> i32{
   usersCount/100000
}

pub fn buildCustomer(row: &Row) -> Customer{
    let name: String = row.get("name").expect("there is no column called name ");
    let phone:  String = row.get("phone").expect("there is no column called phone");
    let segment: String = row.get("segment").expect("there is no column called segment");
    let balance: i32 = row.get("balance").expect("there is no column called balance");
    Customer {name: name, phone: phone, segment: segment, balance: balance}
}