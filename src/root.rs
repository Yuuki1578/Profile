use rocket::response::content;
use maud::{DOCTYPE, html};

#[rocket::get("/")]
pub async fn root() -> content::RawHtml<String> {
    content::RawHtml(html!(
        (DOCTYPE)
        html lang="id" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Mawang Pedia" }
            }

            body {
                h1 { "Hello World!" }
            }
        }
    ).into_string())
}
