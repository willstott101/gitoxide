use std::sync::atomic::AtomicBool;

use gix::Progress;

use crate::{pack, OutputFormat};

/// A general purpose context for many operations provided here
pub struct Context {
    /// If set, provide statistics to `out` in the given format
    pub output_statistics: Option<OutputFormat>,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
    pub verify_mode: pack::verify::Mode,
    pub algorithm: pack::verify::Algorithm,
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub fn integrity(
    repo: gix::Repository,
    mut out: impl std::io::Write,
    progress: impl Progress,
    should_interrupt: &AtomicBool,
    Context {
        output_statistics,
        thread_limit,
        verify_mode,
        algorithm,
    }: Context,
) -> anyhow::Result<()> {
    #[cfg_attr(not(feature = "serde1"), allow(unused))]
    let mut outcome = repo.objects.store_ref().verify_integrity(
        progress,
        should_interrupt,
        gix::odb::pack::index::verify::integrity::Options {
            verify_mode,
            traversal: algorithm.into(),
            thread_limit,
            // TODO: a way to get the pack cache from a handle
            make_pack_lookup_cache: || gix::odb::pack::cache::Never,
        },
    )?;
    if let Some(index) = repo.worktree().map(|wt| wt.index()).transpose()? {
        index.verify_integrity()?;
        index.verify_entries()?;
        index.verify_extensions(true, {
            use gix::odb::FindExt;
            let objects = repo.objects;
            move |oid, buf: &mut Vec<u8>| objects.find_tree_iter(oid, buf).ok()
        })?;
        outcome
            .progress
            .info(format!("Index at '{}' OK", index.path().display()));
    }
    match output_statistics {
        Some(OutputFormat::Human) => writeln!(out, "Human output is currently unsupported, use JSON instead")?,
        #[cfg(feature = "serde1")]
        Some(OutputFormat::Json) => {
            serde_json::to_writer_pretty(
                out,
                &serde_json::json!({
                    "index_statistics" : outcome.index_statistics,
                    "loose_object-stores" : outcome.loose_object_stores
                }),
            )?;
        }
        None => {}
    }
    Ok(())
}
