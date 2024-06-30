mod root;
mod parts;

#[rocket::launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    rocket::Rocket::build()
        .mount("/", rocket::routes![root::root])
}
