use actix_files::NamedFile;
use actix_web::Result;

pub async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

pub async fn about() -> Result<NamedFile> {
    Ok(NamedFile::open("static/about.html")?)
}

pub async fn tracker() -> Result<NamedFile> {
    Ok(NamedFile::open("static/tracker.html")?)
}
