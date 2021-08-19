mod alias;
mod column;

pub use alias::{Alias, Aliasable};
pub use column::Column;

pub type SortExpression = String;
pub type Expression = String;
pub type Condition = String;
pub type OutputExpression = String;
pub type TableName = String;
