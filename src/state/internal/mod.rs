mod engine;

use engine::Engine;

use crate::model::Mesh;

#[derive(Debug, Clone, Default)]
pub struct InternalState {
    engine: Engine,
}
