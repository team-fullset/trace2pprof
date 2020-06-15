use std::collections::HashMap;

use prost::Message;
use string_interner::{DefaultStringInterner, Symbol};

mod proto {
    include!(concat!(env!("OUT_DIR"), "/perftools.profiles.rs"));
}

pub struct Builder {
    strings: DefaultStringInterner,
    location_map: HashMap<u64, proto::Location>,
    profile: proto::Profile,
}

impl Builder {
    pub fn new() -> Self {
        let mut strings = DefaultStringInterner::default();
        strings.get_or_intern(""); // first string must be ""

        let location_map = HashMap::new();

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

        Self {
            strings,
            location_map,
            profile,
        }
    }

    pub fn push_sample_type(&mut self, typ: &str, unit: &str) {
        self.profile.sample_type.push(proto::ValueType {
            r#type: self.strings.get_or_intern(typ).to_usize() as i64,
            unit: self.strings.get_or_intern(unit).to_usize() as i64,
        });
    }

    pub fn push_sample_values(&mut self, addresses: Vec<u64>, values: &[i64]) {
        for address in addresses.iter() {
            self.location_map
                .entry(*address)
                .or_insert_with(|| proto::Location {
                    id: *address,
                    mapping_id: 0,
                    address: *address,
                    line: vec![],
                    is_folded: false,
                });
        }

        let sample = proto::Sample {
            location_id: addresses,
            value: values.to_vec(),
            label: vec![],
        };

        self.profile.sample.push(sample);
    }

    pub fn finish(mut self) -> Vec<u8> {
        for (_, string) in self.strings.iter() {
            self.profile.string_table.push(string.to_owned());
        }

        for (_, location) in self.location_map.into_iter() {
            self.profile.location.push(location);
        }

        let mut encoded_profile = Vec::new();
        self.profile.encode(&mut encoded_profile).unwrap();
        encoded_profile
    }
}
