extern crate chrono;

mod unparse;
mod protocol;
mod types;
mod preprocess;
mod common;

pub mod fix40;
pub mod fix41;
pub mod fix42;
pub mod fix43;
pub mod fix44;
//pub mod fix50;
//pub mod fix50sp1;
//pub mod fix50sp2;

pub use protocol::{FIXVersion, MsgBody};

pub use types::{Data, FIXFloat, Amt, Percentage, Price, Qty, SeqNum, TagNum,
    NumInGroup, Length, FIXBoolean, FIXChar, FIXString, DayOfMonth, MonthYear, UTCDateOnly,
    UTCTimeOnly, UTCTimestamp, TZTimeOnly, TZTimestamp, MultipleValueString, MultipleValueChar,
    XMLData, Reserved1000Plus, Reserved100Plus, Reserved4000Plus, Tenor, Language, Currency,
    Country, ReutersExchange, MICExchange};

pub use preprocess::preprocess as preprocess;
pub use preprocess::PreprocessError;
