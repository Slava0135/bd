use std::vec;

use postgres::Row;
use rand::Rng;

pub enum Entity {
    Station,
    Route,
    RouteSection,
}

pub enum Statement {
    Select(String),
    Delete(String),
    Update(String),
}

pub struct Request {
    pub tables: Vec<String>,
    pub statement: Statement,
    pub entity: Option<Entity>,
}

pub fn row_to_string(row: &Row, result_type: &Entity) -> String {
    match result_type {
        Entity::Station => {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let latitude: f32 = row.get(2);
            let longitude: f32 = row.get(3);
            format!("{id} {name} {latitude} {longitude}")
        },
        Entity::Route => {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let first_station_id: i32 = row.get(2);
            let last_station_id: i32 = row.get(3);
            format!("{id} {name} {first_station_id} {last_station_id}")
        },
        Entity::RouteSection => {
            let id: i32 = row.get(0);
            let route_id: i32 = row.get(1);
            let cost: f32 = row.get(4);
            let departure_station_id: i32 = row.get(5);
            let destination_station_id: i32 = row.get(6);
            format!("{id} {route_id} {departure_station_id} {destination_station_id} {cost}")
        },
    }
}

pub fn random_select() -> Request {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..3) {
        0 => {
            select_all_routes()
        }
        1 => {
            select_all_route_sections()
        }
        _ => {
            select_all_stations()
        }
    }
}

fn select_all_stations() -> Request {
    Request {
        tables: vec!["stations".to_string()],
        statement: Statement::Select("SELECT * FROM stations".to_string()),
        entity: Some(Entity::Station),
    }
}

fn select_all_routes() -> Request {
    Request {
        tables: vec!["routes".to_string()],
        statement: Statement::Select("SELECT * FROM routes".to_string()),
        entity: Some(Entity::Route),
    }
}

fn select_all_route_sections() -> Request {
    Request {
        tables: vec!["route_sections".to_string()],
        statement: Statement::Select("SELECT * FROM route_sections".to_string()),
        entity: Some(Entity::RouteSection),
    }
}