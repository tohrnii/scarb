use anyhow::Result;
use async_trait::async_trait;

use crate::core::manifest::{ManifestDependency, Summary};
use crate::core::package::{Package, PackageId};
pub use id::*;

mod id;

/// Something that finds and downloads remote packages based on names and versions.
#[async_trait]
pub trait Source {
    /// Returns the `SourceId` corresponding to this source.
    fn source_id(&self) -> SourceId;

    /// Attempts to find the packages that match a dependency request.
    async fn query(&mut self, dependency: &ManifestDependency) -> Result<Vec<Summary>>;

    /// Fetches the full package for each name and version specified.
    async fn download(&mut self, id: PackageId) -> Result<Package>;
}
