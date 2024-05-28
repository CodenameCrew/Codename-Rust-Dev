pub fn song_folder(song: &String) -> String {
    return format!("songs/{}/", song.to_lowercase());
}

pub fn chart(song: &String, difficulty:Option<&String>) -> String {
    let lower_difficulty: String = match difficulty {
        Some(diff) => diff.to_lowercase(),
        None => "normal".to_owned()
    };

    return format!("{}charts/{lower_difficulty}.json", song_folder(song));
}

pub fn meta(song: &String, difficulty:Option<Option<&String>>) -> String {
    match difficulty {
        Some(diff) => {
            let lower_difficulty: String = match diff {
                Some(diff) => diff.to_lowercase(),
                None => "normal".to_owned()
            };
            return format!("{}meta-{lower_difficulty}.json", song_folder(song));
        },
        None => return format!("{}meta.json", song_folder(song))
    };
}