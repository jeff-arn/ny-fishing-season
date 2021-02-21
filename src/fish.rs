use chrono::*;
use std::cmp::Ordering;
use tide::prelude::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn season_identifies_active_season() {
    let season = Season {
      start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
      end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
      limit: String::from("5"),
      size: String::from("12\""),
    };

    assert_eq!(
      season.is_active_season(&Local.ymd(2021, 10, 15).and_hms(0, 0, 0)),
      true
    );
  }

  #[test]
  fn season_identifies_inactive_season() {
    let season = Season {
      start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
      end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
      limit: String::from("5"),
      size: String::from("12\""),
    };

    assert_eq!(
      season.is_active_season(&Local.ymd(2021, 10, 25).and_hms(0, 0, 0)),
      false
    );
  }

  #[test]
  fn season_indentifies_upcoming_season() {
    let season = Season {
      start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
      end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
      limit: String::from("5"),
      size: String::from("12\""),
    };

    assert_eq!(
      season.is_upcoming_season(&Local.ymd(2021, 9, 30).and_hms(0, 0, 0)),
      true
    );
  }

  #[test]
  fn season_indentifies_not_upcoming_season() {
    let season = Season {
      start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
      end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
      limit: String::from("5"),
      size: String::from("12\""),
    };

    assert_eq!(
      season.is_upcoming_season(&Local.ymd(2021, 11, 30).and_hms(0, 0, 0)),
      false
    );
  }

  #[test]
  fn fish_gets_active_season() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.get_active_season(&Local.ymd(2021, 10, 15).and_hms(0, 0, 0));
    assert_eq!(result.is_none(), false);
    assert_eq!(
      result.unwrap().start,
      Local.ymd(2021, 10, 10).and_hms(0, 0, 0)
    );

    let result2 = fish.get_active_season(&Local.ymd(2021, 4, 23).and_hms(0, 0, 0));
    assert_eq!(result2.is_none(), false);
    assert_eq!(
      result2.unwrap().start,
      Local.ymd(2021, 4, 10).and_hms(0, 0, 0)
    )
  }

  #[test]
  fn fish_returns_none_for_no_active_season() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.get_active_season(&Local.ymd(2021, 12, 30).and_hms(0, 0, 0));
    assert!(result.is_none());
  }

  #[test]
  fn fish_returns_upcoming_seasons() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.get_upcoming_seasons(&Local.ymd(2021, 1, 30).and_hms(0, 0, 0));
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].start, Local.ymd(2021, 10, 10).and_hms(0, 0, 0));
    assert_eq!(result[1].start, Local.ymd(2021, 4, 10).and_hms(0, 0, 0));

    let result2 = fish.get_upcoming_seasons(&Local.ymd(2021, 5, 30).and_hms(0, 0, 0));
    assert_eq!(result2.len(), 1);
    assert_eq!(result2[0].start, Local.ymd(2021, 10, 10).and_hms(0, 0, 0));
  }

  #[test]
  fn fish_returns_no_upcoming_seasons() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.get_upcoming_seasons(&Local.ymd(2021, 12, 30).and_hms(0, 0, 0));
    assert_eq!(result.is_empty(), true);
  }

  #[test]
  fn fish_returns_closest_season() {
    let filtered_seasons = vec![
      Season {
        start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      },
      Season {
        start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      },
    ];

    let result = Fish::get_closest_season(filtered_seasons);
    assert_eq!(result.is_none(), false);
    assert_eq!(
      result.unwrap().start,
      Local.ymd(2021, 4, 10).and_hms(0, 0, 0)
    )
  }

  #[test]
  fn fish_returns_none_if_no_closest_season() {
    let filtered_seasons = vec![];

    let result = Fish::get_closest_season(filtered_seasons);
    assert!(result.is_none());
  }

  #[test]
  fn fish_returns_next_season() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.get_next_season(&Local.ymd(2021, 3, 30).and_hms(0, 0, 0));
    assert_eq!(result.is_none(), false);
    assert_eq!(
      result.unwrap().start,
      Local.ymd(2021, 4, 10).and_hms(0, 0, 0)
    );

    let result2 = fish.get_next_season(&Local.ymd(2021, 9, 30).and_hms(0, 0, 0));
    assert_eq!(result2.is_none(), false);
    assert_eq!(
      result2.unwrap().start,
      Local.ymd(2021, 10, 10).and_hms(0, 0, 0)
    );
  }

  #[test]
  fn fish_returns_none_if_no_next_season() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.get_next_season(&Local.ymd(2021, 12, 30).and_hms(0, 0, 0));
    assert!(result.is_none());
  }

  #[test]
  fn fish_converts_to_in_season_fish() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.to_in_season_fish(Season {
      start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
      end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
      limit: String::from("5"),
      size: String::from("12\""),
    });

    assert_eq!(result.season_ends, Local.ymd(2021, 10, 20).and_hms(0, 0, 0));
  }

  #[test]
  fn fish_converts_to_upcoming_season_fish() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.to_upcoming_season_fish(Season {
      start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
      end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
      limit: String::from("5"),
      size: String::from("12\""),
    });

    assert_eq!(
      result.next_season_start_date,
      Local.ymd(2021, 10, 10).and_hms(0, 0, 0)
    );
  }

  #[test]
  fn fish_converts_to_out_of_season_fish() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("fish"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let result = fish.to_out_of_season_fish();

    assert_eq!(result.name, fish.name);
  }

  #[test]
  fn fish_category_constructs_from_fish_vec() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("in_season"),
      year_round_fish: false,
      seasons: vec![
        Season {
          start: Local.ymd(2021, 10, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 10, 20).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
        Season {
          start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
          end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
          limit: String::from("5"),
          size: String::from("12\""),
        },
      ],
    };

    let fish2 = Fish {
      id: String::from("test"),
      name: String::from("upcoming_season"),
      year_round_fish: false,
      seasons: vec![Season {
        start: Local.ymd(2021, 11, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 11, 20).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      }],
    };

    let fish3 = Fish {
      id: String::from("test"),
      name: String::from("out_of_season"),
      year_round_fish: false,
      seasons: vec![Season {
        start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 4, 25).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      }],
    };

    let fish_category = FishCategory::from(
      vec![fish, fish2, fish3],
      Local.ymd(2021, 10, 15).and_hms(0, 0, 0),
    );

    assert_eq!(fish_category.in_season.len(), 1);
    assert_eq!(fish_category.upcoming_season.len(), 1);
    assert_eq!(fish_category.out_of_season.len(), 1);

    assert_eq!(fish_category.in_season[0].name, String::from("in_season"));
    assert_eq!(
      fish_category.upcoming_season[0].name,
      String::from("upcoming_season")
    );
    assert_eq!(
      fish_category.out_of_season[0].name,
      String::from("out_of_season")
    );
  }

  #[test]
  fn fish_category_sorts_by_closest_season() {
    let fish = Fish {
      id: String::from("test"),
      name: String::from("in_season"),
      year_round_fish: false,
      seasons: vec![Season {
        start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 6, 20).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      }],
    };

    let fish2 = Fish {
      id: String::from("test"),
      name: String::from("in_season"),
      year_round_fish: false,
      seasons: vec![Season {
        start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 7, 20).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      }],
    };

    let fish3 = Fish {
      id: String::from("test"),
      name: String::from("in_season"),
      year_round_fish: false,
      seasons: vec![Season {
        start: Local.ymd(2021, 4, 10).and_hms(0, 0, 0),
        end: Local.ymd(2021, 8, 20).and_hms(0, 0, 0),
        limit: String::from("5"),
        size: String::from("12\""),
      }],
    };

    let fish_category = FishCategory::from(
      vec![fish, fish2, fish3],
      Local.ymd(2021, 4, 11).and_hms(0, 0, 0),
    );

    assert_eq!(fish_category.in_season.len(), 3);
    assert_eq!(
      fish_category.in_season[0].season_ends,
      Local.ymd(2021, 6, 20).and_hms(0, 0, 0)
    );
    assert_eq!(
      fish_category.in_season[1].season_ends,
      Local.ymd(2021, 7, 20).and_hms(0, 0, 0)
    );
    assert_eq!(
      fish_category.in_season[2].season_ends,
      Local.ymd(2021, 8, 20).and_hms(0, 0, 0)
    );
  }
}

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
pub struct Fish {
  id: String,
  name: String,
  year_round_fish: bool,
  seasons: Vec<Season>,
}

impl Fish {
  fn get_active_season(&self, target_date: &DateTime<Local>) -> Option<Season> {
    self
      .seasons
      .clone()
      .into_iter()
      .find(|s| s.is_active_season(target_date))
  }

  fn get_upcoming_seasons(&self, target_date: &DateTime<Local>) -> Vec<Season> {
    self
      .seasons
      .clone()
      .into_iter()
      .filter(|s| s.is_upcoming_season(target_date))
      .collect()
  }

  fn get_closest_season(seasons: Vec<Season>) -> Option<Season> {
    let closest_season = seasons.iter().min_by(|s0, s1| s0.start.cmp(&s1.start))?;
    Some(closest_season.clone())
  }

  fn get_next_season(&self, target_date: &DateTime<Local>) -> Option<Season> {
    let upcoming_seasons = self.get_upcoming_seasons(target_date);
    Self::get_closest_season(upcoming_seasons)
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
pub struct FishCategory {
  in_season: Vec<InSeasonFish>,
  upcoming_season: Vec<UpcomingSeasonFish>,
  out_of_season: Vec<OutOfSeasonFish>,
}

impl FishCategory {
  pub fn from(fish: Vec<Fish>, target_date: DateTime<Local>) -> Self {
    let mut in_season_vec = Vec::new();
    let mut upcoming_season_vec = Vec::new();
    let mut out_of_season_vec = Vec::new();

    for f in fish.iter() {
      match f.get_active_season(&target_date) {
        Some(active_season) => in_season_vec.push(f.to_in_season_fish(active_season)),
        None => match f.get_next_season(&target_date) {
          Some(next_season) => upcoming_season_vec.push(f.to_upcoming_season_fish(next_season)),
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

  pub fn sort_by_closest_season(&mut self) -> &mut FishCategory {
    self
      .in_season
      .sort_by(|s0, s1| s0.season_ends.cmp(&s1.season_ends));

    self
      .upcoming_season
      .sort_by(|s0, s1| s0.next_season_start_date.cmp(&s1.next_season_start_date));

    self
  }
}
