mod option_catch_all;

use rocket::Route;
use option_catch_all::all_options;

pub fn create_routes() -> Vec<Route> {
    routes![all_options]
}
