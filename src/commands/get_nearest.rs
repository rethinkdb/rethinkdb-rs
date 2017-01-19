command! {
    /// Return a list of documents closest to a specified point based on a geospatial index, sorted
    /// in order of increasing distance

    #[command(get_nearest(args(point = "T")))]
}
