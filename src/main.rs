use chrono::*;
use serde_json;
use std::fs;
use tera::Tera;
use tide::prelude::*;
use tide_tera::prelude::*;

mod fish;

#[derive(Serialize, Deserialize)]
struct Document {
    saltwater: Vec<fish::Fish>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut tera = Tera::new("src/templates/*.html")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    app.at("/styles").serve_dir("public/styles/")?;
    app.at("/scripts").serve_dir("public/scripts/")?;

    app.at("/").get(|req: tide::Request<Tera>| async move {
        let tera = req.state();

        let json = fs::read_to_string("src/data/dec.json")?;
        let document: Document = serde_json::from_str(&json)?;

        let mut fish: fish::FishCategory =
            fish::FishCategory::from(document.saltwater, Local::now());
        fish.sort_by_closest_season();

        tera.render_response(
            "test.html",
            &context! {
                "fish" => fish,
            },
        )
    });

    app.listen("127.0.0.1:4000").await?;

    Ok(())
}
