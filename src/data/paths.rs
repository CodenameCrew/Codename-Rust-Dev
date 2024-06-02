use std::path::Path;


pub fn inst(song: &String, difficulty:Option<&String>) -> String {
    return song_file("Inst", song, difficulty, None);
}

pub fn voices(song: &String, difficulty:Option<&String>, suffix: Option<&String>) -> String {
    return song_file("Voices", song, difficulty, suffix);
}

#[inline(always)]
pub fn song_file(file: &str, song: &String, difficulty:Option<&String>, suffix: Option<&String>) -> String {
    if difficulty != None {
        let difficulty_path: String = format!(
            "{}song/Voices{}-{}.ogg", 
            song_folder(song), suffix_string(suffix), 
            difficulty_string(difficulty).to_lowercase()
        );
        if exists(&difficulty_path) {return difficulty_path;}
    }

    return format!("{}song/{file}{}.ogg", song_folder(song), suffix_string(suffix));
}

pub fn chart(song: &String, difficulty:Option<&String>) -> String {
    return format!("{}charts/{}.json", song_folder(song), difficulty_string(difficulty));
}

pub fn meta(song: &String, difficulty:Option<Option<&String>>) -> String {
    match difficulty {
        Some(diff) => return format!("{}meta-{}.json", song_folder(song), difficulty_string(diff)),
        None => return format!("{}meta.json", song_folder(song))
    };
}

#[inline(always)]
pub fn song_folder(song: &String) -> String {
    return format!("songs/{}/", song.to_lowercase());
}

#[inline(always)]
pub fn suffix_string(suffix:Option<&String>) -> String {
    return match suffix {
        Some(string) => string.to_owned(),
        None => "".to_owned()
    };
}

#[inline(always)]
pub fn difficulty_string(difficulty:Option<&String>) -> String {
    return match difficulty {
        Some(diff) => diff.to_lowercase(),
        None => "normal".to_owned()
    };
}

#[inline(always)]
pub fn exists(path: &String) -> bool {
    return Path::new(path).exists();
}