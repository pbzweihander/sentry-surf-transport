use std::sync::Arc;

use sentry::{ClientOptions, Transport, TransportFactory};

use crate::surf::SurfHttpTransport;

pub fn factory(options: &ClientOptions) -> Arc<dyn Transport> {
    Arc::new(SurfHttpTransport::new(options))
}

pub fn make_factory() -> Arc<dyn TransportFactory> {
    Arc::new(factory)
}
