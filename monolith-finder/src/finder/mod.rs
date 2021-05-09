use crate::coord::SamplePos2D;
use crate::finder::search_constraint::{SearchConstraint, SearchResult};
use crate::noise::SamplingJob;
use crate::util::DerefSliceArrayVal;
use crate::worldgen::ChunkGenerator;

mod search_constraint;

// Always samples until the constraint can be determined for *all* elements
pub fn search<J, T>(job: &mut J, constraint: &T) -> Option<T::Found>
where
    J: SamplingJob,
    T: SearchConstraint<J>,
{
    loop {
        let result = job
            .results()
            .iter()
            .map(|v| constraint.is_found(job, *v))
            .fold(SearchResult::NotFound, |acc, v| acc.and(v));
        match result {
            SearchResult::NotFound => break None,
            SearchResult::Found(t) => break Some(t),
            SearchResult::Unknown => job.sample_once(),
        }
    }
}

pub struct PointResult {
    pub is_candidate: bool,
    pub is_land: bool,
}

pub fn inspect_point(chunk_gen: &ChunkGenerator, pos: SamplePos2D) -> PointResult {
    let mut hill_job = chunk_gen
        .hill_noise()
        .sample2d(pos, 1, 1, DerefSliceArrayVal([0.0]));
    let is_candidate = search(&mut hill_job, &search_constraint::less_constraint(-512.0)).is_some();
    let mut depth_job = chunk_gen
        .depth_noise()
        .sample2d(pos, 1, 1, DerefSliceArrayVal([0.0]));
    let is_land = search(
        &mut depth_job,
        &search_constraint::absolute_greater_equals(8000.0),
    )
    .is_some();
    PointResult {
        is_candidate,
        is_land,
    }
}
