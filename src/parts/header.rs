use maud::html;
use rocket::response::content;

pub async fn header() -> content::RawHtml<String> {
    content::RawHtml(html!(
        script {
            alert("Hello World!");
        }
    ).into_string())
}
