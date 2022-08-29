use serde::{Serialize, Deserialize};

pub trait Cost2 {
    fn calculate(&self) -> f32;
}

pub struct MonthlyCost {
    cost_per_unit: f32,
    monthly_rate_of_usage: f32
}

impl Cost2 for MonthlyCost {
    fn calculate(&self) -> f32 {
        self.cost_per_unit * self.monthly_rate_of_usage
    }
}

impl MonthlyCost {
    pub fn new(cost_per_unit: f32, monthly_rate_of_usage: f32) -> MonthlyCost {
        MonthlyCost {
            cost_per_unit,
            monthly_rate_of_usage
        }
    }
}

pub struct Currency {
    cents: u16,
    dollar: u32,
}

impl Currency {
    pub fn new(value: f32) -> Currency {
        Currency {
            dollar: (value / 1.0) as u32,
            cents: (value % 1.0) as u16,
        }
    }

    pub fn to_string(&self) -> String {
        format!("${}.{}", self.dollar, self.cents)
    }
}

/**
 * Main Cost stuff for Yaml File
 */
#[derive(Debug, Deserialize)]
struct FinanceUnit {
    name: String,
    amount: f32,
    frequency: Frequency
}

#[derive(Debug, Deserialize)]
struct Income {
    name: String,
    amount: f32,
    frequency: Frequency
}

#[derive(Debug, Deserialize)]
struct IncomeGroup {
    incomes: Vec<FinanceUnit>
}

#[derive(Debug, Deserialize)]
struct CostGroup {
    name: String,
    expenses: Vec<FinanceUnit>
}

#[derive(Debug, Deserialize)]
pub struct FinancialInfo {
    income: Vec<FinanceUnit>,
    expenses: Vec<CostGroup>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Frequency {
    Weekly,
    Daily,
    Monthly,
    Bimonthly,
    Biweekly,
    Yearly
}

pub trait CalculateFinance {
    fn monthly_expenses_print(&self, print_cost: bool) -> f32;

    fn monthly_expenses(&self) -> f32 {
        self.monthly_expenses_print(false)
    }

    fn yearly_expenses(&self) -> f32 {
        self.monthly_expenses() * 12 as f32
    }

    fn get_10_month_emergency_fund(&self) -> f32 {
        self.monthly_expenses() * 10 as f32
    }

    fn monthly_income(&self) -> f32;
    fn yearly_income(&self) -> f32 {
        self.monthly_income() * 12 as f32
    }
    fn monthly_leftovers(&self) -> f32 {
        self.monthly_income() - self.monthly_expenses()
    }


}

fn get_cost_as_monthly(cost: &FinanceUnit) -> f32 {
    match cost.frequency {
        Frequency::Bimonthly => cost.amount / 2.,
        Frequency::Weekly => cost.amount * 4.,
        Frequency::Daily => cost.amount * 30.,
        Frequency::Yearly => cost.amount / 12.,
        Frequency::Biweekly => cost.amount * 2.,
        Frequency::Monthly => cost.amount,
    }
}

impl CalculateFinance for FinancialInfo {
    fn monthly_expenses_print(&self, print_cost: bool) -> f32 {
        let mut monthly_expense: f32 = 0 as f32;
        for cost_group in self.expenses.iter() {
            let mut cost_group_expense: f32 = 0.;
            for cost in cost_group.expenses.iter() {
                cost_group_expense = cost_group_expense + get_cost_as_monthly(cost);
            }
            if print_cost {
                println!("{}: ${:.2}", cost_group.name, cost_group_expense);
            }
            monthly_expense = monthly_expense + cost_group_expense;
        }

        monthly_expense
    }

    fn monthly_income(&self) -> f32 {
        let mut monthly_income: f32 = 0 as f32;
        for income in self.income.iter() {
            monthly_income = monthly_income + get_cost_as_monthly(income)
        }

        monthly_income
    }
}


