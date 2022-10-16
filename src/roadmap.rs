use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};

use chrono::NaiveDate;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::contributor::Contributor;
use crate::parse_date;

static MAX_ESTIMATED_COMPLEXITY: f64 = 5.0;
static MIN_ESTIMATED_COMPLEXITY: f64 = 0.0;

static MAX_COMPLEXITY_SCORE: f64 = 1.0;
static MIN_COMPLEXITY_SCORE: f64 = 0.0;

static MAX_ESTIMATED_VALUE: f64 = 5.0;
static MIN_ESTIMATED_VALUE: f64 = 0.0;

static TARGET_DATE_FACTOR: f64 = 0.2;
static DURATION_FACTOR: f64 = 0.1;
static COMPLEXITY_FACTOR: f64 = 0.3;
static VALUE_FACTOR: f64 = 0.4;

#[derive(Debug, Clone, Deserialize, Validate)]
#[validate(schema(function = "validate_dates", skip_on_field_errors = false))]
pub struct RoadmapItem {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(range(min = 1, max = 5, message = "Complexity must be between 1 and 5"))]
    pub estimated_complexity: usize,
    #[validate(range(min = 1, max = 5, message = "Value must be between 1 and 5"))]
    pub estimated_value: usize,
    pub start_date: NaiveDate,
    pub target_date: NaiveDate,
    pub urgency: Option<f64>,
    pub contributors: Option<Vec<Contributor>>,
}

impl RoadmapItem {
    pub fn new(
        name: String,
        estimated_complexity: usize,
        estimated_value: usize,
        start_date: NaiveDate,
        target_date: NaiveDate,
        contributors: Vec<Contributor>,
    ) -> RoadmapItem {
        return RoadmapItem {
            name,
            estimated_complexity,
            estimated_value,
            start_date,
            target_date,
            urgency: Some(calculate_project_urgency(
                estimated_complexity,
                estimated_value,
                start_date,
                target_date,
            )),
            contributors: Some(contributors),
        };
    }

    pub fn add_contributors(mut self, contributors: Vec<Contributor>) {
        self.contributors = Some(contributors);
    }

    pub fn get_urgency(&self) -> f64 {
        self.urgency.unwrap_or_else(|| 0.0)
    }

    pub fn get_contributors(&self) -> Vec<Contributor> {
        self.contributors.clone().unwrap_or_else(|| Vec::new())
    }

    pub fn update_urgency(&mut self) {
        self.urgency = Some(calculate_project_urgency(
            self.estimated_complexity,
            self.estimated_value,
            self.start_date,
            self.target_date,
        ));
    }
}

fn calculate_project_urgency(
    estimated_complexity: usize,
    estimated_value: usize,
    start_date: NaiveDate,
    target_date: NaiveDate,
) -> f64 {
    // 1. Target date - the closer in time the more urgent
    let today = chrono::offset::Local::today().naive_utc();
    let days_from_today =
        TARGET_DATE_FACTOR / target_date.signed_duration_since(today).num_days() as f64;
    println!("Days from today: {days_from_today}");

    // 2. Duration - the shorter the more urgent
    let project_duration =
        DURATION_FACTOR / target_date.signed_duration_since(start_date).num_days() as f64;
    println!("Duration: {project_duration}");

    // 3. Estimated complexity - the higher the more urgent
    let complexity = (estimated_complexity as f64 - MIN_ESTIMATED_COMPLEXITY)
        / (MAX_ESTIMATED_COMPLEXITY - MIN_ESTIMATED_COMPLEXITY)
        * COMPLEXITY_FACTOR;
    println!("Complexity: {complexity}");

    // Estimated value - the more value added the more urgent
    let value = (estimated_value as f64 - MIN_ESTIMATED_VALUE)
        / (MAX_ESTIMATED_VALUE - MIN_ESTIMATED_VALUE)
        * VALUE_FACTOR;
    println!("Value: {value}");

    let total = days_from_today + project_duration + complexity + value;
    println!("Total: {total}");

    (f64::max(
        0.0,
        f64::min(
            1.0,
            (total - MIN_COMPLEXITY_SCORE) / (MAX_COMPLEXITY_SCORE - MIN_COMPLEXITY_SCORE),
        ),
    ) * 100.0)
        .round()
        / 100.0
}

impl Ord for RoadmapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.urgency > other.urgency {
            return Ordering::Less;
        }

        if other.urgency > self.urgency {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for RoadmapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RoadmapItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self
                .start_date
                .signed_duration_since(other.start_date)
                .is_zero()
            && self
                .target_date
                .signed_duration_since(other.target_date)
                .is_zero()
            && self
                .get_contributors()
                .iter()
                .all(|contributor| other.get_contributors().contains(contributor))
    }
}

impl Eq for RoadmapItem {}

impl Display for RoadmapItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{0},{1},{2},{3},{4}",
            self.name,
            self.start_date,
            self.target_date,
            self.urgency.unwrap_or_else(|| 0.0),
            self.get_contributors()
                .iter()
                .map(|contributor| format!("{}", contributor.name))
                .fold(String::new(), |acc, arg| acc + arg.as_str() + ";")
        )
    }
}

fn validate_dates(item: &RoadmapItem) -> Result<(), ValidationError> {
    let date_diff = item
        .target_date
        .signed_duration_since(item.start_date)
        .num_days();
    if date_diff.is_negative() {
        return Err(ValidationError::new(
            "The target date cannot be before the start date.",
        ));
    }

    let today = chrono::offset::Local::today().naive_utc();
    if item
        .target_date
        .signed_duration_since(today)
        .num_days()
        .is_negative()
    {
        return Err(ValidationError::new(
            "The target date cannot be before today.",
        ));
    }

    return Ok(());
}

// Use this to play around with the complexity formula
#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::roadmap::calculate_project_urgency;

    #[test]
    fn it_works() {
        let result = calculate_project_urgency(
            5,
            5,
            NaiveDate::from_ymd(2022, 10, 15),
            NaiveDate::from_ymd(2022, 10, 16),
        );
        println!("{result}")
    }
}
