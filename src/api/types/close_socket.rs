pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CloseSocket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_socket: Option<bool>,
}

impl CloseSocket {
    pub fn builder() -> CloseSocketBuilder {
        <CloseSocketBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloseSocketBuilder {
    close_socket: Option<bool>,
}

impl CloseSocketBuilder {
    pub fn close_socket(mut self, value: bool) -> Self {
        self.close_socket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CloseSocket`].
    pub fn build(self) -> Result<CloseSocket, BuildError> {
        Ok(CloseSocket {
            close_socket: self.close_socket,
        })
    }
}
