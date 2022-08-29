use core::cost::cost_core::{MonthlyCost, Cost};

#[test]
fn should_calculate_monthly_cost() {
    let monthly_cost: MonthlyCost = MonthlyCost::new(20.0, 30.0);
    assert_eq!(monthly_cost.calculate(), 600.0)
}