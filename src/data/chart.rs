
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChartStrumLine {
    pub characters : Vec<String>,
    pub r#type : i32,
    pub notes : Vec<ChartNote>,
    pub position : String,
    pub visible : Option<bool>,
    pub strumPos : Option<Vec<f32>>,

}

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