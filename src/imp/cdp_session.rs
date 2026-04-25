use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct CdpSession {
    channel: ChannelOwner,
}

impl CdpSession {
    pub(crate) fn new(channel: ChannelOwner) -> Self {
        Self { channel }
    }

    pub(crate) async fn send(
        &self,
        method: &str,
        params: Option<&Value>,
    ) -> Result<Value, Arc<Error>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            method: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            params: Option<&'a Value>,
        }
        let args = Args { method, params };
        let v = send_message!(self, "send", args);
        // Playwright wraps the CDP response in `{ result: ... }`.
        let result = v
            .as_object()
            .and_then(|m| m.get("result"))
            .cloned()
            .unwrap_or(Value::Null);
        Ok(result)
    }

    pub(crate) async fn detach(&self) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "detach", Map::new());
        Ok(())
    }
}

impl RemoteObject for CdpSession {
    fn channel(&self) -> &ChannelOwner {
        &self.channel
    }
    fn channel_mut(&mut self) -> &mut ChannelOwner {
        &mut self.channel
    }
}
