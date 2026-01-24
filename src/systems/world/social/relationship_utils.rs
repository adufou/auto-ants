/// Applies a tanh-based modifier to relationship changes based on current relationship state.
/// Creates echo chamber dynamics where similar relationships amplify changes.
///
/// # Examples
/// - At relationship = 0.0: modifier = 0% (neutral, no amplification)
/// - At relationship = ±0.5: modifier = ±76%
/// - At relationship = ±0.9: modifier = ±97%
///
/// For positive changes (gains):
/// - Positive relationships: amplifies the gain
/// - Negative relationships: reduces the gain
///
/// For negative changes (losses):
/// - Positive relationships: reduces the loss
/// - Negative relationships: amplifies the loss
pub fn apply_tanh_modifier(current_relationship: f32, base_change: f32) -> f32 {
    let modifier = (2.0 * current_relationship).tanh();

    if base_change >= 0.0 {
        // Positive change: amplify if relationship is positive
        base_change * (1.0 + modifier)
    } else {
        // Negative change: amplify if relationship is negative
        base_change * (1.0 - modifier)
    }
}

/// Calculate selection weight for a relationship topic using tanh(2x).
/// Stronger opinions (positive or negative) have higher selection probability.
///
/// # Examples
/// - relationship = 0.0 → weight = 0.0 (neutral, rarely discussed)
/// - relationship = ±0.5 → weight = 0.76
/// - relationship = ±0.9 → weight = 0.97
///
/// Using absolute value means both love and hate are equally "interesting" topics.
pub fn calculate_topic_weight(relationship: f32) -> f32 {
    (2.0 * relationship.abs()).tanh()
}
