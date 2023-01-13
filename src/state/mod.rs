pub mod inner;
pub mod shared;

use inner::InnerState;
use shared::SharedState;

struct State<'shared> {
    inner: InnerState,
    shared: &'shared SharedState,
}

impl<'shared> State<'shared> {}
