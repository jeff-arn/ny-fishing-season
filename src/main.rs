use chrono::*;
use serde_json;
use std::cmp::Ordering;
use std::fs;
use tera::Tera;
use tide::prelude::*;
use tide_tera::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
struct Season {
    start: DateTime<Local>,
    end: DateTime<Local>,
    limit: String,
    size: String,
}

impl Season {
    fn is_active_season(&self, target_date: &DateTime<Local>) -> bool {
        target_date.cmp(&self.start) == Ordering::Greater
            && target_date.cmp(&self.end) == Ordering::Less
    }

    fn is_upcoming_season(&self, target_date: &DateTime<Local>) -> bool {
        target_date.cmp(&self.start) == Ordering::Less
    }
}

#[derive(Serialize, Deserialize)]
struct Fish {
    id: String,
    name: String,
    year_round_fish: bool,
    seasons: Vec<Season>,
}

impl Fish {
    fn get_active_season(&self, target_date: &DateTime<Local>) -> Option<Season> {
        self.seasons
            .clone()
            .into_iter()
            .find(|s| s.is_active_season(target_date))
    }

    fn get_upcoming_seasons(&self, target_date: &DateTime<Local>) -> Vec<Season> {
        self.seasons
            .clone()
            .into_iter()
            .filter(|s| s.is_upcoming_season(target_date))
            .collect()
    }

    fn get_closest_season(&self, seasons: Vec<Season>) -> Option<Season> {
        let closest_season = seasons.iter().min_by(|s0, s1| s0.start.cmp(&s1.start))?;
        Some(closest_season.clone())
    }

    fn get_next_season(&self, target_date: &DateTime<Local>) -> Option<Season> {
        let upcoming_seasons = self.get_upcoming_seasons(target_date);
        self.get_closest_season(upcoming_seasons)
    }

    fn to_in_season_fish(&self, active_season: Season) -> InSeasonFish {
        InSeasonFish {
            id: self.id.to_string(),
            name: self.name.to_string(),
            limit: active_season.limit,
            size: active_season.size,
            year_round_fish: self.year_round_fish,
            season_ends: active_season.end,
        }
    }

    fn to_upcoming_season_fish(&self, upcoming_season: Season) -> UpcomingSeasonFish {
        UpcomingSeasonFish {
            id: self.id.to_string(),
            name: self.name.to_string(),
            next_season_start_date: upcoming_season.start,
        }
    }

    fn to_out_of_season_fish(&self) -> OutOfSeasonFish {
        OutOfSeasonFish {
            id: self.id.to_string(),
            name: self.name.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct InSeasonFish {
    id: String,
    name: String,
    limit: String,
    size: String,
    year_round_fish: bool,
    season_ends: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
struct UpcomingSeasonFish {
    id: String,
    name: String,
    next_season_start_date: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
struct OutOfSeasonFish {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct FishCategory {
    in_season: Vec<InSeasonFish>,
    upcoming_season: Vec<UpcomingSeasonFish>,
    out_of_season: Vec<OutOfSeasonFish>,
}

impl FishCategory {
    fn from(fish: Vec<Fish>, target_date: DateTime<Local>) -> Self {
        let mut in_season_vec = Vec::new();
        let mut upcoming_season_vec = Vec::new();
        let mut out_of_season_vec = Vec::new();

        for f in fish.iter() {
            match f.get_active_season(&target_date) {
                Some(active_season) => in_season_vec.push(f.to_in_season_fish(active_season)),
                None => match f.get_next_season(&target_date) {
                    Some(next_season) => {
                        upcoming_season_vec.push(f.to_upcoming_season_fish(next_season))
                    }
                    None => out_of_season_vec.push(f.to_out_of_season_fish()),
                },
            }
        }

        FishCategory {
            in_season: in_season_vec,
            upcoming_season: upcoming_season_vec,
            out_of_season: out_of_season_vec,
        }
    }

    fn sort_by_closest_season(&mut self) -> &mut FishCategory {
        self.in_season
            .sort_by(|s0, s1| s0.season_ends.cmp(&s1.season_ends));

        self.upcoming_season
            .sort_by(|s0, s1| s0.next_season_start_date.cmp(&s1.next_season_start_date));

        self
    }
}

#[derive(Serialize, Deserialize)]
struct Document {
    saltwater: Vec<Fish>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut tera = Tera::new("src/templates/*.html")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    app.at("/styles").serve_dir("public/styles/")?;

    app.at("/").get(|req: tide::Request<Tera>| async move {
        let tera = req.state();

        let json = fs::read_to_string("src/data/dec.json")?;
        let document: Document = serde_json::from_str(&json)?;

        let mut fish: FishCategory = FishCategory::from(document.saltwater, Local::now());
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
