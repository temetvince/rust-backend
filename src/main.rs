use dal::crud_service::CrudService;
use populate_db::print_db;
use sqlx::Error;

mod dal;
mod populate_db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let crud_service = CrudService::new().await;

    populate_db::populate_db(&crud_service).await?;

    print_db(&crud_service).await?;

    Ok(())
}
