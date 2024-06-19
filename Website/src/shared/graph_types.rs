use std::fmt::Display;

use serde::{Deserialize, Serialize};
use num_derive::{*};
#[cfg_attr(feature="ssr", derive(sqlx::Type))]
#[derive(Clone,Serialize,Deserialize,Debug,PartialEq,Eq,ToPrimitive,FromPrimitive)]
#[repr(i32)]
pub enum GraphTypes {
    BasicLine,
    SmoothedLine,
    BasicBar,
}


impl Display for GraphTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphTypes::BasicLine => write!(f,"Basic Line Graph"),
            GraphTypes::SmoothedLine => write!(f,"Smoothed Line Graph"),
            GraphTypes::BasicBar => write!(f,"Basic Bar Graph"),
        }
    }
}