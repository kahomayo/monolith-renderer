use crate::coord::{BlockPos2D, SamplePos2D};
use crate::finder::search_constraint::{SearchConstraint, SearchResult};
use crate::noise::SamplingJob;
use crate::util::DerefSliceArrayVal;
use crate::worldgen::ChunkGenerator;

mod search_constraint;

const WORLD_BORDER: i32 = 32_000_000;
const FAR_LANDS: i32 = 12_550_824; // Slight approximation, would have to consult actual noise gen for 821 vs 824 in positive dir

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

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum PointResult {
    Land,
    Water,
    Monolith,
    FarLands(FarLandsKind),
    OOB,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum FarLandsKind {
    X,
    Z,
    Corner,
}

/// Determines the terrain at a given position
pub fn inspect_point(chunk_gen: &ChunkGenerator, pos: SamplePos2D) -> PointResult {
    let block_pos = BlockPos2D::from(pos);
    match (block_pos.x.abs(), block_pos.z.abs()) {
        (x, z) if x >= WORLD_BORDER || z >= WORLD_BORDER => return PointResult::OOB,
        (x, z) if x >= FAR_LANDS && z >= FAR_LANDS => return PointResult::FarLands(FarLandsKind::Corner),
        (x, _) if x >= FAR_LANDS => return PointResult::FarLands(FarLandsKind::X),
        (_, z) if z >= FAR_LANDS => return PointResult::FarLands(FarLandsKind::Z),
        (_, _) => (),
    }

    let mut depth_job = chunk_gen
        .depth_noise()
        .sample2d(pos, 1, 1, DerefSliceArrayVal([0.0]));
    let is_land = search(
        &mut depth_job,
        &search_constraint::absolute_greater_equals(8000.0),
    ).is_some();
    if !is_land {
        return PointResult::Water;
    }

    let mut hill_job = chunk_gen
        .hill_noise()
        .sample2d(pos, 1, 1, DerefSliceArrayVal([0.0]));
    let is_candidate = search(&mut hill_job, &search_constraint::less_constraint(-512.0)).is_some();
    if !is_candidate {
        return PointResult::Land;
    }

    PointResult::Monolith
}
