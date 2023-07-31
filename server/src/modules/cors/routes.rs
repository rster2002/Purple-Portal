mod option_catch_all;

use option_catch_all::all_options;
use rocket::Route;

pub fn create_routes() -> Vec<Route> {
    routes![all_options]
}
