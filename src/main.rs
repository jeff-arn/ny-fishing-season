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

        let today = Local::now();

        let mut fish: fish::FishCategory = fish::FishCategory::from(document.saltwater, today);
        fish.sort_by_closest_season();

        tera.render_response(
            "test.html",
            &context! {
                "fish" => fish,
                "target_date" => today,
            },
        )
    });

    app.at("/:year/:month/:day")
        .get(|req: tide::Request<Tera>| async move {
            let tera = req.state();

            let json = fs::read_to_string("src/data/dec.json")?;
            let document: Document = serde_json::from_str(&json)?;

            let year = req.param("year")?;
            let month = req.param("month")?;
            let day = req.param("day")?;

            let parsed_year = year.parse::<i32>();
            let parsed_month = month.parse::<u32>();
            let parsed_day = day.parse::<u32>();

            let target_date = Local
                .ymd(parsed_year?, parsed_month?, parsed_day?)
                .and_hms(0, 0, 0);

            let mut fish: fish::FishCategory =
                fish::FishCategory::from(document.saltwater, target_date);
            fish.sort_by_closest_season();

            tera.render_response(
                "test.html",
                &context! {
                    "fish" => fish,
                    "target_date" => target_date,
                },
            )
        });

    app.listen("0.0.0.0:4000").await?;

    Ok(())
}
