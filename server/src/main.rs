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
                    <base href=\"/\" />
                    <link rel=\"stylesheet\" href=\"css/styles.css\"/>
                    <link rel=\"manifest\" href=\"./manifest.webmanifest\"/>
                    <script>
                        window.onload = () => {
                            'use strict';
                            navigator.serviceWorker.register('./sw.js')
                        }
                    </script>
                    <script type=\"module\">
                        import init, { run } from './about_client.js';
                        const start = async() => {
                            await init('./about_client_bg.wasm');
                            run();
                        };
                        start();
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

mod static_files {
    use rocket::fs::{relative, NamedFile};
    use std::path::Path;

    #[get("/manifest.webmanifest")]
    pub async fn manifest() -> Option<NamedFile> {
        let path = Path::new(relative!("../about_client/manifest.webmanifest"));
        NamedFile::open(path).await.ok()
    }

    #[get("/css/styles.css")]
    pub async fn css() -> Option<NamedFile> {
        let path = Path::new(relative!("../about_client/css/styles.css"));
        NamedFile::open(path).await.ok()
    }
    #[get("/sw.js")]
    pub async fn service_worker() -> Option<NamedFile> {
        let path = Path::new(relative!("../about_client/js/sw.js"));
        NamedFile::open(path).await.ok()
    }

    #[get("/about_client.js")]
    pub async fn client() -> Option<NamedFile> {
        let path = Path::new(relative!("../about_client/pkg/about_client.js"));
        NamedFile::open(path).await.ok()
    }

    #[get("/about_client_bg.wasm")]
    pub async fn wasm() -> Option<NamedFile> {
        let path = Path::new(relative!("../about_client/pkg/about_client_bg.wasm"));
        NamedFile::open(path).await.ok()
    }

    #[get("/bundle")]
    pub async fn bundle() -> Option<NamedFile> {
        let path = Path::new(relative!("../about_client/js/about_client.js"));
        NamedFile::open(path).await.ok()
    }
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build().mount(
        &config::ROCKET::BASE_URL(),
        rocket::routes![
            root,
            static_files::service_worker,
            static_files::css,
            static_files::manifest,
            static_files::client,
            static_files::wasm,
            static_files::bundle
        ],
    )
}
