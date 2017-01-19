command! {
    /// Convert a [GeoJSON](http://geojson.org) object to a ReQL geometry object

    #[command(geojson(args(geojson = "T")))]
}
