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
                    <meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\"/>
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
                    <title>Overengineered</title>
                    <script>
                        window.onload = () =-> {
                            'use strict';
                            navigator.serviceWorker.register('./sw.js')
                        }
                    </script>
                    <script type=\"module\">
                        import init, { run } from './about_client.js';
                        const start = async() => {
                            await init('./about_client.wasm');
                            run()
                        };
                    </script>
                </head>
                <body>
                    Hello from raw html
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
