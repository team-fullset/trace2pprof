use prost::Message;
use string_interner::{DefaultStringInterner, Symbol};

mod proto {
    include!(concat!(env!("OUT_DIR"), "/perftools.profiles.rs"));
}

pub struct Sample {
    pub location_id: u64,
    pub value: i64,
}

pub struct Function<'a> {
    pub id: u64,
    pub name: &'a str,
    pub system_name: &'a str,
    pub filename: &'a str,
    pub start_line: i64,
}

pub struct Location {
    pub id: u64,
    pub address: u64,
    pub lines: Vec<Line>,
}

pub struct Line {
    pub function_id: u64,
    pub line: i64,
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

    pub fn push_sample(&mut self, sample: Sample) {
        self.profile.sample.push(proto::Sample {
            location_id: vec![sample.location_id],
            value: vec![sample.value],
            label: vec![],
        });
    }

    pub fn push_function(&mut self, function: Function) {
        self.profile.function.push(proto::Function {
            id: function.id,
            name: self.strings.get_or_intern(function.name).to_usize() as i64,
            system_name: self.strings.get_or_intern(function.system_name).to_usize() as i64,
            filename: self.strings.get_or_intern(function.filename).to_usize() as i64,
            start_line: function.start_line,
        });
    }

    pub fn push_location(&mut self, location: Location) {
        self.profile.location.push(proto::Location {
            id: location.id,
            mapping_id: 0,
            address: location.address,
            line: location
                .lines
                .iter()
                .map(|line| proto::Line {
                    function_id: line.function_id,
                    line: line.line,
                })
                .collect(),
            is_folded: false,
        });
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
