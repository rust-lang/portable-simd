/// A SIMD vector.
pub trait Vector {
    /// The data type contained by the vector.
    type Scalar;

    /// The number of data elements contained by the vector.
    const LANES: usize;
}
