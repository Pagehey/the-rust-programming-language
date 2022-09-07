use std::collections::HashMap;

use tokio;
use tokio_postgres::{NoTls, Error};

#[derive(Debug)]
struct Order {
    id: i64,
    expense_code_id: i64,
    total_amount_cents: i32
}

impl Order {
    fn total_amount(&self) -> f32 {
        self.total_amount_cents as f32 / 100.0
    }
}

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=sunday_development", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    // let rows = client
    //     .query("SELECT $1::TEXT", &[&"hello world"])
    //     .await?;

    // // And then check that we got back the same string we sent over.
    // let value: &str = rows[0].get(0);
    // assert_eq!(value, "hello world");

    let rows = client.query("SELECT id, expense_code_id, total_amount_cents FROM orders WHERE project_id = 6;", &[]).await?;

    let orders: Vec<Order> = rows.iter().map(|row| {
        Order {
            id: row.get("id"),
            expense_code_id: row.get("expense_code_id"),
            total_amount_cents: row.get("total_amount_cents")
        }
    }).collect();

    let mut orders_by_expense_code_id: HashMap<i64, Vec<&Order>> = HashMap::new();

    orders.iter().for_each(|order| {
        orders_by_expense_code_id.entry(order.expense_code_id).or_default().push(order);
    });

    orders_by_expense_code_id.iter().for_each(|(expense_code_id, orders)| {
        let sum = orders.iter().fold(0 as f32, |acc, order| acc + order.total_amount());
        println!("Expense code with id {} has a total order amount of {}", expense_code_id, sum);
    });


    Ok(())
}
