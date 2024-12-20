use sqlx::Error;
use tokio::fs;

use crate::dal::{crud_service::CrudService, models::{Group, Type, Ware}};

pub async fn populate_db(crud_service: &CrudService) -> Result<(), Error> {
    // Get data from JSON files
    let groups = get_structs_from_json_file::<Group>("data/groups.json").await?;
    let types = get_structs_from_json_file::<Type>("data/types.json").await?;
    let wares = get_structs_from_json_file::<Ware>("data/wares.json").await?;

    // Insert data into the database
    crud_service.create(&groups).await?;
    crud_service.create(&types).await?;
    crud_service.create(&wares).await?;   
    
    Ok(())
}

pub async fn print_db(crud_service: &CrudService) -> Result<(), Error> {
    let all_groups = crud_service.list::<Group>().await?;
    let all_types = crud_service.list::<Type>().await?;
    let all_wares = crud_service.list::<Ware>().await?;

    println!("Groups:\n{}", serde_json::to_string_pretty(&all_groups).unwrap_or_else(|_| "Failed to format groups".to_string()));
    println!("Types:\n{}", serde_json::to_string_pretty(&all_types).unwrap_or_else(|_| "Failed to format types".to_string()));
    println!("Wares:\n{}", serde_json::to_string_pretty(&all_wares).unwrap_or_else(|_| "Failed to format wares".to_string()));

    Ok(())
}

async fn get_structs_from_json_file<T>(file: &str) -> Result<Vec<T>, Error> where T: serde::de::DeserializeOwned {
    let json_data = fs::read_to_string(file).await?;
    let data: Vec<T> = serde_json::from_str(&json_data).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    Ok(data)
}
