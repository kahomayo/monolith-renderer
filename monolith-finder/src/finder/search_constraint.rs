use crate::noise::SamplingJob;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SearchResult<T> {
    NotFound,
    Unknown,
    Found(T),
}

impl<T> SearchResult<T> {
    pub fn or(self, other: Self) -> Self {
        match self {
            SearchResult::Found(_) => self,
            SearchResult::Unknown => {
                if let SearchResult::Found(_) = other {
                    other
                } else {
                    self
                }
            }
            SearchResult::NotFound => other,
        }
    }

    pub fn and(self, other: Self) -> Self {
        match self {
            SearchResult::Found(_) => {
                if let SearchResult::Unknown = other {
                    other
                } else {
                    self
                }
            }
            SearchResult::Unknown => self,
            SearchResult::NotFound => other,
        }
    }
}

pub trait SearchConstraint<Job: SamplingJob> {
    type Found;

    fn is_found(&self, job: &Job, value: f64) -> SearchResult<Self::Found>;
}

struct ComparisonConstraint<F, FInv>
where
    F: Fn(f64, f64) -> bool,
    FInv: Fn(f64, f64) -> bool,
{
    bound: f64,
    f: F,
    f_inv: FInv,
}

// impl<F, FInv> ComparisonConstraint<F, FInv> {}

pub fn less_equals_constraint<Job: SamplingJob>(bound: f64) -> impl SearchConstraint<Job> {
    ComparisonConstraint {
        bound,
        f: |dist, rem| dist <= -rem,
        f_inv: |dist, rem| dist > rem,
    }
}

pub fn less_constraint<Job: SamplingJob>(bound: f64) -> impl SearchConstraint<Job> {
    ComparisonConstraint {
        bound,
        f: |dist, rem| dist < -rem,
        f_inv: |dist, rem| dist >= rem,
    }
}

pub fn greater_constraint<Job: SamplingJob>(bound: f64) -> impl SearchConstraint<Job> {
    ComparisonConstraint {
        bound,
        f: |dist, rem| dist > rem,
        f_inv: |dist, rem| dist <= -rem,
    }
}

pub fn greater_equals_constraint<Job: SamplingJob>(bound: f64) -> impl SearchConstraint<Job> {
    ComparisonConstraint {
        bound,
        f: |dist, rem| dist >= rem,
        f_inv: |dist, rem| dist < -rem,
    }
}

pub fn absolute_greater_equals<Job: SamplingJob>(bound: f64) -> impl SearchConstraint<Job> {
    AbsoluteGeqConstraint { bound }
}

struct AbsoluteGeqConstraint {
    bound: f64,
}

impl<Job: SamplingJob> SearchConstraint<Job> for AbsoluteGeqConstraint {
    type Found = ();

    fn is_found(&self, job: &Job, value: f64) -> SearchResult<Self::Found> {
        let lowest_possible = value.abs() - job.remaining_variation();
        let highest_possible = value.abs() + job.remaining_variation();
        if lowest_possible <= self.bound {
            SearchResult::Found(())
        } else if self.bound <= highest_possible || !value.is_normal() {
            SearchResult::NotFound
        } else {
            SearchResult::Unknown
        }
    }
}

impl<F, FInv, Job: SamplingJob> SearchConstraint<Job> for ComparisonConstraint<F, FInv>
where
    F: Fn(f64, f64) -> bool,
    FInv: Fn(f64, f64) -> bool,
{
    type Found = ();

    fn is_found(&self, job: &Job, value: f64) -> SearchResult<Self::Found> {
        let dist_to_bound = value - self.bound;
        let rem_variation = job.remaining_variation();
        if (self.f)(dist_to_bound, rem_variation) {
            SearchResult::Found(())
        } else if (self.f_inv)(dist_to_bound, rem_variation) || !dist_to_bound.is_normal() {
            SearchResult::NotFound
        } else {
            SearchResult::Unknown
        }
    }
}
