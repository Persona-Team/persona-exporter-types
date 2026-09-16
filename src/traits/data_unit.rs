/// A list of various units of information measurement:
/// bytes, kilobytes, megabytes, gigabytes, terabytes, petabytes.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataUnit {
    Bytes,
    KiloBytes,
    MegaBytes,
    GigaBytes,
    TeraBytes,
    PetaBytes,
}

impl DataUnit {
    /// It is called on function arguments of type [`DataUnit`]
    /// and returns a value in bytes.
    ///
    /// # Example
    /// Let's look at an example of converting a 100-gigabyte swap
    /// space and convert those 100 gigabytes into megabytes.
    /// see also [`convert_from_to`] function
    /// ```rust
    /// use persona_exporter_types::types::DataUnit;
    ///
    /// fn main() {
    ///     let swap = 100;
    ///
    ///     let from = DataUnit::GigaBytes.as_bytes();
    ///     let to = DataUnit::MegaBytes.as_bytes();
    ///
    ///     let total_swap = (swap as f64 * from) / to;
    ///
    ///     assert_eq!(from, 1024.0 * 1024.0 * 1024.0);
    ///     assert_eq!(to, 1024.0 * 1024.0);
    ///     assert_eq!(total_swap, 102400.0)
    /// }
    /// ```
    pub fn as_bytes(&self) -> f64 {
        match self {
            DataUnit::Bytes => 1.0,
            DataUnit::KiloBytes => 1024.0,
            DataUnit::MegaBytes => 1024.0 * 1024.0,
            DataUnit::GigaBytes => 1024.0 * 1024.0 * 1024.0,
            DataUnit::TeraBytes => 1024.0 * 1024.0 * 1024.0 * 1024.0,
            DataUnit::PetaBytes => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        }
    }
}

fn convert_from_to(data: f64, from: DataUnit, to: DataUnit) -> f64 {
    let bytes = data * from.as_bytes();
    bytes / to.as_bytes()
}
/// Data conversion between different units of measurement
/// # Arguments
/// * `from` - We specify the unit of our input data.
///   This allows us to pass bytes, kilobytes, megabytes, etc.,
///   to the function.
/// * `to` - Specify the unit to which you want to convert the data.
/// # Example
/// ```rust
/// use persona_exporter_types::types::{DataUnit, ConvertTo};
///
/// fn main() {
///     let total_RAM = 16543600640u64;
///     let gigabytes_RAM = total_RAM
///         .to_dataunit(
///             DataUnit::Bytes,
///             DataUnit::GigaBytes
///         );
///
///     assert_eq!(gigabytes_RAM, 15.407428741455078);
/// }
/// ```
pub trait ConvertTo {
    fn to_dataunit(self, from: DataUnit, to: DataUnit) -> f64;
}

macro_rules! insert_convert_to_impl {
    ( $( $t:ty ),* ) => {
        $(
            impl ConvertTo for $t {
                #[doc = concat!["Converting a value of type **", stringify!($t), "** from one unit to another" ]]
                /// For more information about conversition, see [`ConvertTo`]
                fn to_dataunit(self, from: DataUnit, to: DataUnit) -> f64 {
                    convert_from_to(self as f64, from, to)
                }
            }
        )*
    };
}

insert_convert_to_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
