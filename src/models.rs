extern crate serde;

#[derive(Debug)]
pub enum MetricType {
    Status
}

impl serde::Serialize for MetricType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
        match *self {
            MetricType::Status => serializer.serialize_str("status")
        }
    }
}

#[derive(Debug)]
pub enum DataStatus {
    Ok,
    Warning,
    Error
}

impl serde::Serialize for DataStatus {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
        match *self {
            DataStatus::Ok      => serializer.serialize_str("ok"),
            DataStatus::Warning => serializer.serialize_str("warning"),
            DataStatus::Error   => serializer.serialize_str("error")
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MetricData {
    pub status: DataStatus,
}

#[derive(Serialize, Debug)]
pub struct Metric {
    pub key: String,
    #[serde(rename="type")]
    pub metric_type: MetricType,
    pub data: MetricData
}

impl Metric {
    pub fn new(value: i32) -> Metric {
        Metric {
            key: "nodeping".to_string(),
            metric_type: MetricType::Status,
            data: MetricData {
                status:
                    match value {
                        0 => DataStatus::Ok,
                        _ => DataStatus::Error,
                    }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusPing {
    pub value: i32
}
