use postgres::{ Client, NoTls, Error };

struct Person<'a> {
    id: Option<u32>,
    name: &'a str,
    data: &'a [u8]
}

impl<'a> Person<'a> {
    fn id(&self) -> u32 {
        self.id.unwrap_or_default()
    }
}

fn main() -> Result<(), Error> {
    // let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let person = Person { id: None, name: "toto", data: &[1] };

    println!("{:?}", person.id());
    // client.batch_execute("
    //     CREATE TABLE person (
    //         id      SERIAL PRIMARY KEY,
    //         name    TEXT NOT NULL,
    //         data    BYTEA
    //     )
    // ")?;

    // let name = "Ferris";
    // let data = None::<&[u8]>;
    // client.execute(
    //     "INSERT INTO person (name, data) VALUES ($1, $2)",
    //     &[&name, &data],
    // )?;

    // for row in client.query("SELECT id, name, data FROM person", &[])? {
    //     let id: u32 = row.get(0);
    //     let name = row.get(1);
    //     let data: Option<&[u8]> = row.get(2);

    //     println!("found person: {} {} {:?}", id, name, data);
    // }

    Ok(())
}
