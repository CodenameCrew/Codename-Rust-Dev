
use bevy::render::color::Color;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[path = "stage.rs"] mod stage;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartData {
	pub strumLines: Vec<ChartStrumLine>,
	pub events: Vec<ChartEvent>,
    #[serde(default = "default_meta")]
	pub meta: Option<ChartMetaData>,
	pub codenameChart: bool,
	pub stage: Option<String>,
	pub scrollSpeed: f64,
	pub noteTypes: Vec<String>,

	pub fromMods: Option<bool>,
}
pub fn default_meta() -> Option<ChartMetaData> {None}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartMetaData {
    #[serde(default = "default_string")]
    pub name: Option<String>,
    #[serde(default = "default_f64")]
    pub bpm: Option<f64>,
    #[serde(default = "default_string")]
    pub displayName: Option<String>,
    #[serde(default = "default_f64")]
	pub beatsPerMeasure: Option<f64>,
    #[serde(default = "default_f64")]
	pub stepsPerBeat: Option<f64>,
    #[serde(default = "default_string")]
	pub icon: Option<String>,
    #[serde(default = "default_value")]
	pub color: Option<Value>,
    #[serde(default = "default_string_vec")]
	pub difficulties: Option<Vec<String>>,
    #[serde(default = "default_bool")]
	pub coopAllowed: Option<bool>,
    #[serde(default = "default_bool")]
	pub opponentModeAllowed: Option<bool>,
    #[serde(default = "default_value")]
	pub customValues: Option<Value>,

    #[serde(default = "default_color")]
    #[serde(skip)]
	pub parsedColor: Option<Color>,
}

pub fn default_f64() -> Option<f64> {None}
pub fn default_string() -> Option<String> {None}
pub fn default_value() -> Option<Value> {None}
pub fn default_string_vec() -> Option<Vec<String>> {None}
pub fn default_bool() -> Option<bool> {None}
pub fn default_color() -> Option<Color> {None}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartStrumLine {
    pub characters : Vec<String>,
    pub r#type : i32,
    pub notes : Vec<ChartNote>,
    pub position : String,
    #[serde(default = "default_bool")]
    pub visible : Option<bool>,
    #[serde(default = "default_f32_vec")]
    pub strumPos : Option<Vec<f32>>,
}

pub fn default_f32_vec() -> Option<Vec<f32>> {None}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartNote {
    pub time: f64,
    pub id: i32,
    pub r#type: i32,
    pub sLen: f64
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartEvent {
    name: String,
    time: f64,
    params: Vec<Value>
}

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartStrumLineType;

impl ChartStrumLineType {
    /**
	 * STRUMLINE IS MARKED AS OPPONENT - WILL BE PLAYED BY CPU, OR PLAYED BY PLAYER IF OPPONENT MODE IS ON
	*/
    pub const OPPONENT: i32 = 0;
    /**
	 * STRUMLINE IS MARKED AS PLAYER - WILL BE PLAYED AS PLAYER, OR PLAYED AS CPU IF OPPONENT MODE IS ON
	*/
    pub const PLAYER: i32 = 1;
    /**
	 * STRUMLINE IS MARKED AS ADDITIONAL - WILL BE PLAYED AS CPU EVEN IF OPPONENT MODE IS ENABLED
	*/
    pub const ADDITIONAL: i32 = 2;
}

pub const BASE_CHART: ChartData = ChartData {
    strumLines: vec![],
    events: vec![],
    meta: None,
    codenameChart: true,
    stage: None,
    scrollSpeed: 1.0,
    noteTypes: vec![],
    fromMods: None
};

pub mod parser {
    use bevy::render::color::Color;
    use serde_json::from_str;
    use serde_json::Map;
    use crate::paths;
    use super::{ChartData, ChartMetaData};

    pub fn parse(song: &String, difficulty: Option<&String>) -> Option<ChartData> {
        let chart_path: String = paths::chart(&song, difficulty);
        let file: Result<String, std::io::Error> = std::fs::read_to_string("assets/".to_owned() + &chart_path);

        match file {
            Ok(f) => {
                let json:Result<ChartData, serde_json::Error> = from_str(&f);
                match json {
                    Ok(mut chart) => {
                        match chart.meta {
                            Some(meta) => {chart.meta = Some(meta_defaults(song, meta))}
                            None => {if let Some(meta) = parse_meta(&song) {chart.meta = Some(meta)}}
                        }

                        chart.fromMods = Some(false);
                        return Some(chart);
                    },
                    Err(e) => {println!("ERROR PARSING CHART {song}: {}", e); return None;}
                }
            }
            Err(e) => {println!("ERROR GETTING FILE {}: {}", &chart_path, e); return None;}
        }
    }

    pub fn parse_meta(song: &String) -> Option<ChartMetaData> {
        let meta_path: String = paths::meta(&song, None);
        let file: Result<String, std::io::Error> = std::fs::read_to_string("assets/".to_owned() + &meta_path);

        match file {
            Ok(f) => {
                let json:Result<ChartMetaData, serde_json::Error> = from_str(&f);
                match json {
                    Ok(meta) => return Some(meta_defaults(&song, meta)),
                    Err(e) => {println!("ERROR PARSING META {song}: {}", e); return None;}
                }
            }
            Err(e) => {println!("ERROR GETTING FILE {}: {}", &meta_path, e); return None;}
        }
    }

    pub fn meta_defaults(song: &String, mut meta: ChartMetaData) -> ChartMetaData {
        if meta.name == None {meta.name = Some(song.clone())}
        if meta.bpm == None {meta.bpm = Some(100.0)}
        if meta.displayName == None {meta.displayName = Some(song.clone())}
        if meta.beatsPerMeasure == None {meta.beatsPerMeasure = Some(4.0)}
        if meta.stepsPerBeat == None {meta.stepsPerBeat = Some(4.0)}
        if meta.icon == None {meta.icon = Some("face".to_owned())}

        if meta.color == None {meta.color = Some(serde_json::Value::String("#9271FD".to_owned()))}
        meta.parsedColor = match meta.color.clone().unwrap() {
            serde_json::Value::String(color_string) => Some(Color::hex(color_string).unwrap()),
            _ => Some(Color::rgb(0.0, 0.0, 0.0))
        };

        if meta.difficulties == None {
            let charts_path: String = "assets/".to_owned() + &paths::song_folder(&song) + "charts/";
            let directory: Result<std::fs::ReadDir, std::io::Error> = std::fs::read_dir(&charts_path);
            match directory {
                Ok(files) => {
                    let mut json_charts: Vec<String> = vec![];
                    for file in files {
                        match file {
                            Ok(entry) => {
                                let file_path: std::path::PathBuf = entry.path();
                                let ext: String = file_path.extension().unwrap().to_str().unwrap().to_owned();
                                let name: String = file_path.file_stem().unwrap().to_str().unwrap().to_owned();
                                
                                if ext == "json" {json_charts.push(name);}
                            },
                            Err(e) => println!("ERROR READING DIRECTORY ENTRY: {}", e)
                        }
                    }
                    meta.difficulties = Some(json_charts);
                },
                Err(e) => {
                    println!("ERROR READING DIRECTORY {}: {}", &charts_path, e);
                    meta.difficulties = Some(vec![]);
                }
            }
        }
        
        if meta.coopAllowed == None {meta.coopAllowed = Some(true)}
        if meta.opponentModeAllowed == None {meta.coopAllowed = Some(true)}
        if meta.customValues == None {meta.customValues = Some(serde_json::Value::Object(Map::new()))}

        return meta;
    }
}