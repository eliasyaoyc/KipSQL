pub mod display;
pub mod logical_create_table_plan;
pub mod logical_plan_builder;
pub mod logical_select_plan;
pub mod operator;

use self::{
    logical_create_table_plan::LogicalCreateTablePlan, logical_select_plan::LogicalSelectPlan,
};
use thiserror::__private::DisplayAsDisplay;

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalPlan {
    Select(LogicalSelectPlan),
    CreateTable(LogicalCreateTablePlan),
}
