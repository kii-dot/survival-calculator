mod cost;

use std::fs;
use std::iter::Map;
use core::cost::cost_core::{MonthlyCost};
use serde::{Serialize, Deserialize};
use serde_yaml;
use num_format::{Locale, ToFormattedString};
use crate::cost::cost_core::{FinancialInfo, CalculateFinance, Currency};

fn main() {
    let locale = Locale::en;

    let financial_file = fs::read_to_string("finance-info.yaml").expect(
        "Is file even there?"
    );
    let financial_info: FinancialInfo = serde_yaml::from_str(&financial_file).unwrap();
    println!("monthly income: ${:.2}", financial_info.monthly_income());
    println!();
    println!("total monthly expenses: ${:.2}", financial_info.monthly_expenses_print(true));
    println!();
    println!("monthly leftover: ${:.2}", financial_info.monthly_leftovers());
    println!("10 months emergency funds: ${:.2}", financial_info.get_10_month_emergency_fund());
}

