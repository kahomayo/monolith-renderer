use crate::coord::SamplePos2D;
use crate::finder::search_constraint::{SearchConstraint, SearchResult};
use crate::noise::SamplingJob;
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

pub fn search_monoliths(
    chunk_gen: &ChunkGenerator,
    pos: SamplePos2D,
    res_x: usize,
    res_z: usize,
) -> Vec<bool> {
    let mut hill_job = chunk_gen.hill_noise().sample2d(pos, res_x, res_z);
    let has_candidates =
        search(&mut hill_job, &search_constraint::less_constraint(-512.0)).is_some();
    if has_candidates {
        let mut depth_job = chunk_gen.depth_noise().sample2d(pos, res_x, res_z);
        let has_land = search(
            &mut depth_job,
            &search_constraint::absolute_greater_equals(8000.0),
        )
        .is_some();

        if has_land {
            return hill_job
                .results()
                .iter()
                .zip(depth_job.results().iter())
                .map(|(h, d)| d.abs() >= 8000.0 && *h < -512.0)
                .collect();
        }
    }
    vec![false; res_x * res_z]
}
