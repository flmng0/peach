fn main() {
    use euclid::default;

    type Point = default::Point2D<f32>;
    type Vector = default::Vector2D<f32>;
    type Transform = default::Transform3D<f32>;

    let point = Point::new(2.0, 3.0);
    let transform = Transform::identity();

    let vector = point.to_3d().to_vector();
    let position = transform.transform_vector3d(vector).to_point().to_2d();

    dbg!(point, position);
}
