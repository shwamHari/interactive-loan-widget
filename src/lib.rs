use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Loan {
    principal: f64,
    annual_interest_rate: f64,
    term_years: u32,
}

#[derive(Serialize)]
pub struct AmortizationEntry {
    year: u32,
    payment: f64,
    principal_paid: f64,
    interest_paid: f64,
    remaining_balance: f64,
}

impl Loan {
    pub fn new(principal: f64, annual_interest_rate: f64, term_years: u32) -> Self {
        Self {
            principal,
            annual_interest_rate,
            term_years,
        }
    }

    pub fn yearly_payment(&self) -> f64 {
        let yearly_interest_rate = self.annual_interest_rate / 100.0;
        if yearly_interest_rate == 0.0 {
            self.principal / self.term_years as f64
        } else {
            let denominator = (1.0 + yearly_interest_rate).powi(self.term_years as i32) - 1.0;
            self.principal * yearly_interest_rate * (1.0 + yearly_interest_rate).powi(self.term_years as i32) / denominator
        }
    }

    pub fn amortization_schedule(&self) -> Vec<AmortizationEntry> {
        let yearly_payment = self.yearly_payment();
        let yearly_interest_rate = self.annual_interest_rate / 100.0;
        let mut balance = self.principal;
        let mut schedule = Vec::new();

        for year in 1..=self.term_years {
            let interest_paid = balance * yearly_interest_rate;
            let principal_paid = yearly_payment - interest_paid;
            balance -= principal_paid;

            schedule.push(AmortizationEntry {
                year,
                payment: yearly_payment,
                principal_paid,
                interest_paid,
                remaining_balance: if balance > 0.0 { balance } else { 0.0 },
            });
        }
        schedule
    }
}

#[wasm_bindgen]
pub fn calculate_yearly_payment(principal: f64, annual_interest_rate: f64, term_years: u32) -> Result<f64, JsValue> {
    if principal <= 0.0 {
        return Err(JsValue::from_str("Principal must be positive"));
    }
    if annual_interest_rate < 0.0 {
        return Err(JsValue::from_str("Interest rate cannot be negative"));
    }
    if term_years == 0 {
        return Err(JsValue::from_str("Term must be at least one year"));
    }
    let loan = Loan::new(principal, annual_interest_rate, term_years);
    Ok(loan.yearly_payment())
}

#[wasm_bindgen]
pub fn get_amortization_schedule(principal: f64, annual_interest_rate: f64, term_years: u32) -> Result<JsValue, JsValue> {
    if principal <= 0.0 {
        return Err(JsValue::from_str("Principal must be positive"));
    }
    if annual_interest_rate < 0.0 {
        return Err(JsValue::from_str("Interest rate cannot be negative"));
    }
    if term_years == 0 {
        return Err(JsValue::from_str("Term must be at least one year"));
    }
    let loan = Loan::new(principal, annual_interest_rate, term_years);
    let schedule = loan.amortization_schedule();
    Ok(serde_wasm_bindgen::to_value(&schedule)?)
}
