use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::*;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

pub(crate) const DEFAULT_EMPTY_MESSAGE: &str = "empty";
pub trait FromWithMeasurement<T> {
    fn from_with_name(value: T, measurement: &str) -> Self;
}

pub trait IntoWithMeasurement<T> {
    // type Target;
    fn into_with_name(self, measurement: &str) -> T;
}

pub trait FinishLineProtocol {
    fn finish(self, timestamp: i64) -> Vec<u8>;
}

impl FinishLineProtocol for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn finish(self, timestamp: i64) -> Vec<u8> {
        self.timestamp(timestamp).close_line().build()
    }
}

impl<T, U> IntoWithMeasurement<U> for T
where
    U: FromWithMeasurement<T>,
{
    fn into_with_name(self, measurement: &str) -> U {
        U::from_with_name(self, measurement)
    }
}




