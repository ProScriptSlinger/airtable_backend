use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::repository::{surrealdb_repo::SurrealDBRepo};
use crate::model::table_model::{Table, TableBMC, TablePatch};

#[post("/tables")]
pub async fn create_table(db: Data<SurrealDBRepo>, new_table: Json<Table>) -> HttpResponse {
    let data = Table {
        id: None,
        title: new_table.title.to_owned(),
        body: new_table.body.to_owned(),
    };
    
    let table_detail = TableBMC::create(db, "table", data ).await;

    match table_detail {
         Ok(table) => HttpResponse::Ok().json(table),
         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/tables/{id}")]
pub async fn get_table(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    
    let table_detail = TableBMC::get(db, &id).await;
    
    match table_detail {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/tables/{id}")]
pub async fn update_table(
    db: Data<SurrealDBRepo>,
    path: Path<String>,
    table_patch: Json<TablePatch>,
) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = TablePatch {
        title: table_patch.title.to_owned(),
        body: table_patch.body.to_owned(),
    };
    
    let update_result = TableBMC::update(db, &id, data).await;
    
    match update_result {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/tables/{id}")]
pub async fn delete_table(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    
    let result = TableBMC::delete(db, &id).await;
    
    match result {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/tables")]
pub async fn get_tables(db: Data<SurrealDBRepo>) -> HttpResponse {
    let result = TableBMC::get_all(db).await;
    
    match result {
        Ok(tables) => HttpResponse::Ok().json(tables),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
   }
}