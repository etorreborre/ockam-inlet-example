use ockam::Message;
use serde::{Deserialize, Serialize};

#[derive(Message, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    RegisterUI,
    RegisterClient,
}

#[derive(Message, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Inbound(Command),
    Outbound(Command),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::Command::RegisterUI;
    use crate::Event::Inbound;

    #[test]
    fn test_serialization() {
        let actual = serde_json::to_string(&Inbound(RegisterUI)).unwrap();
        let expected: String = r#"{"Inbound":"RegisterUI"}"#.into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialization() {
        let actual = serde_json::from_str::<Event>(r#"{"Inbound":"RegisterUI"}"#).unwrap();
        let expected = Inbound(RegisterUI);
        assert_eq!(actual, expected);
    }
}
