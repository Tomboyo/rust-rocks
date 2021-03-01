pub trait DynamicPosition {
    /// Translate this entity by applying its own velocity to its current
    /// position.
    fn translate(&mut self);

    /// Constrain the position of this entity such that 0 <= x < modx and
    /// 0 <= y < mody.
    fn clamp(&mut self, modx: f32, mody: f32);
}
