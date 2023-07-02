use bb8_postgres::tokio_postgres::NoTls;
use vinted_rs::{
    db::DbController,
    model::{brand::Brand, filter::Filter},
};

use vinted_rs::VintedWrapper;

const DB_URL: &str = "postgres://postgres:postgres@localhost/vinted-rs";
const POOL_SIZE: u32 = 5;

#[tokio::main]
async fn main() {
    let mut vinted = VintedWrapper::new();

    let filter: Filter = Filter::builder().search_text(String::from("shoes")).build();

    let items = vinted.get_item(filter).await.unwrap();

    print!("{items:?}");

    let db: DbController<NoTls> = DbController::new(DB_URL, POOL_SIZE, NoTls).await.unwrap();

    let brand_name = String::from("adidas");

    let b: Brand = db.get_brand_by_name(&brand_name).await.unwrap();

    let brands = db.get_brands_by_name(brand_name).await.unwrap();

    println!("\n\n\n\nBrand {b:?}\n\n\n\n");

    println!("Brands:  {brands:?}");
}
