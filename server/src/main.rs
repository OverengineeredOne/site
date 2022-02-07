#[macro_use]
extern crate rocket;

itconfig::config! {
    ROCKET {
        static BASE_URL: String => "/",
    }
}

mod static_files {
    use rocket::fs::{relative, NamedFile};
    use std::path::{Path, PathBuf};

    #[get("/")]
    pub async fn root() -> Option<NamedFile> {
        let path = Path::new(relative!("../overengineered-client/dist/index.html"));
        NamedFile::open(path).await.ok()
    }

    #[get("/<file..>")]
    pub async fn static_files(file: PathBuf) -> Option<NamedFile> {
        let path = Path::new(relative!("../overengineered-client/dist"));
        NamedFile::open(path.join(file)).await.ok()
    }
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build().mount(
        &config::ROCKET::BASE_URL(),
        rocket::routes![static_files::root, static_files::static_files,],
    )
}
