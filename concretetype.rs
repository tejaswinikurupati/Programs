struct point<f32> {
    x:f32,
    y:f32,

}

impl point<f32> {
    fn distance_from_origin(&self) ->f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}
