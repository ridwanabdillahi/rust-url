use serde_json::Value;

pub trait JsonExt {
    fn take_key(&mut self, key: &str) -> Option<Value>;
    fn string(self) -> String;
    fn maybe_string(self) -> Option<String>;
    fn take_string(&mut self, key: &str) -> String;
}

impl JsonExt for Value {
    fn take_key(&mut self, key: &str) -> Option<Value> {
        self.as_object_mut().unwrap().remove(key)
    }

    fn string(self) -> String {
        self.maybe_string().expect("")
    }

    fn maybe_string(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            Value::Null => None,
            _ => panic!("Not a Value::String or Value::Null"),
        }
    }

    fn take_string(&mut self, key: &str) -> String {
        self.take_key(key).unwrap().string()
    }
}
