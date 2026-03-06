// Helper function to count truthy values
pub fn count_truthy(values: &[bool]) -> usize {
    values.iter().filter(|&&v| v).count()
}
