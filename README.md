# Survival Calculator

The goal of this app is to provide a quick solution to calculate the cost of living without having to deal with a complicated design.

## Goals
Users should be able to input some basic numbers like, rent, price of ingredients, and utilities, which the program will use to calculate the "supposed" cost of living per month, emergency funds goal (default to 10 months).

## Non-goals
Users to be able to enter how much they'd like to save (to buy a house or whatever) and their supposed income goal to hit to achieve the goals.

## Design
The basics to cost of living/survival are:
1. Shelter
2. Nutrition
3. Utilities** & Income Expenses**

** Utilities: These would include payments required for shelter and work. For example, you would need water, and fire to survive. And electricity provide light source and power to carry out tasks. 

** Income Expenses: In our current society, how we make money is directly correlated to shelter and nutrition. Therefore, the expenses required to produce income is directly correlated to survival.

### Utilities
Utilities normally includes
1. Water
2. Gas
3. Electricity
4. Internet (optional for sure)

Income expenses may include
1. Transportation cost

## Calculation of costs
Let's start with **Utilities**

All recurring expenses will be calculated based on this equation:
```rust
let cost_per_month: f32 = cost_per_unit * rate_of_usage_per_month;
```

For example, electricity can be calculated as
```rust
let cost_per_kwh_electricity: f32 = 0.5; // $0.50 CAD/kwh
let monthly_usage_rate: f32 = 100; // Using 100 kwh per month
let electricity_cost: f32 = cost_per_kwh_electricity * monthly_usage_rate;
```
This can be applied to water, gas, transporation and even food (if applicable).

#### Ultimate Equation
```rust
cost_of_living_per_month = 
    rent_per_month +
    cost_of_food_per_month +
    cost_of_utilities_per_month;

cost_of_living_per_year = cost_of_living_per_month * 12; 
```

## Emergency funds
I think emergency funds are more helpful if we know what we are spending on instead of just a lump sum. In fact all of these should be saved in a structure manner.
```rust
struct CostOfLiving {
    shelter: f32,
    food: f32,
    utilities: CostOfUtilities
}

struct CostOfUtilities {
    water: f32,
    gas: f32,
    electricity: f32,
    internet: f32,
    transportation: f32
}

struct MonthlyCost {
    cost_per_unit: f32,
    monthly_rate_of_usage: f32
}

impl MonthlyCost {
    fn get(&self) -> f32 {
        self.cost_per_unit * self.monthly_rate_of_usage
    }
}
```