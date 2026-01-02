use url::Url;

use crate::{Error, Result};

pub(crate) fn endpoint(base_url: &Url, segments: &[&str]) -> Result<Url> {
    let mut url = base_url.clone();

    if url.query().is_some() || url.fragment().is_some() {
        return Err(Error::invalid_config(
            "base_url must not include query or fragment",
        ));
    }

    {
        let mut path = url.path_segments_mut().map_err(|_| {
            Error::invalid_config("base_url must be a hierarchical URL (e.g. https://host/)")
        })?;

        path.pop_if_empty();
        for segment in segments {
            path.push(segment);
        }
    }

    Ok(url)
}
