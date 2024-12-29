pub mod api;
mod db;
mod seed;

use db::load_all_foods;

pub fn init() {
    load_all_foods();
}
