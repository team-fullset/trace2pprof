use prost::Message;
use string_interner::{DefaultStringInterner, Symbol};

mod proto {
    include!(concat!(env!("OUT_DIR"), "/perftools.profiles.rs"));
}

pub struct Builder {
    strings: DefaultStringInterner,
    profile: proto::Profile,
}

impl Builder {
    pub fn new() -> Self {
        let mut strings = DefaultStringInterner::default();
        strings.get_or_intern(""); // first string must be ""

        let profile = proto::Profile {
            sample_type: vec![],
            sample: vec![],
            mapping: vec![],
            location: vec![],
            function: vec![],
            string_table: vec![],
            drop_frames: 0,
            keep_frames: 0,
            time_nanos: 0,
            duration_nanos: 0,
            period_type: None,
            period: 0,
            comment: vec![],
            default_sample_type: 0,
        };

        Self { strings, profile }
    }

    pub fn push_sample_type(&mut self, typ: &str, unit: &str) {
        self.profile.sample_type.push(proto::ValueType {
            r#type: self.strings.get_or_intern(typ).to_usize() as i64,
            unit: self.strings.get_or_intern(unit).to_usize() as i64,
        });
    }

    pub fn push_sample_values(&mut self, address: u64, values: &[i64]) {
        self.profile.location.push(proto::Location {
            id: address,
            mapping_id: 0,
            address,
            line: vec![],
            is_folded: false,
        });

        let mut sample = proto::Sample {
            location_id: vec![],
            value: vec![],
            label: vec![],
        };

        for value in values {
            sample.location_id.push(address);
            sample.value.push(*value);
        }

        self.profile.sample.push(sample);
    }

    pub fn finish(mut self) -> Vec<u8> {
        for (_, string) in self.strings.iter() {
            self.profile.string_table.push(string.to_owned());
        }

        let mut encoded_profile = Vec::new();
        self.profile.encode(&mut encoded_profile).unwrap();
        encoded_profile
    }
}
