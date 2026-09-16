pub trait Clear {
    /// Clear only fields with dynamic types (String, Vec, HashMap, etc.)
    /// There is no point in clearing simple types (int, float, bool, enum), as they will be overwritten anyway.
    fn clear_dynamic(&mut self);
}
