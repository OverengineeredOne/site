#[macro_use]
extern crate rocket;

use rocket::response::content;

itconfig::config! {
    ROCKET {
        static BASE_URL: String => "/",
    }
}

#[get("/")]
async fn root() -> content::Html<String> {
    content::Html(
        "<!DOCKTYPE html>
            <html>
                <head>
                </head>
                <body>
                    Hello
                </body>
            </html>
        "
        .to_string(),
    )
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build().mount(&config::ROCKET::BASE_URL(), rocket::routes![root,])
}
