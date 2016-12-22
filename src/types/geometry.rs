use types::data;

pub trait IntoGeometry {
    fn into_geometry(self) -> data::Geometry;
}
