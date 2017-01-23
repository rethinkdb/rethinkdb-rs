command! {
    /// Get all documents where the given geometry object intersects the geometry object of the
    /// requested geospatial index

    #[command(get_intersecting(args(args = "T")))]
}
