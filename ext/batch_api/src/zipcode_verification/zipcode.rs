#[derive(Debug)]
pub struct ZipcodeSector {
    name: String,
    geometry: geo::GeometryCollection,
    location: Location,
}

impl ZipcodeSector {
    pub fn new(name: String, geometry: geo::GeometryCollection, location: Location) -> Self {
        ZipcodeSector {
            name,
            geometry,
            location,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn geometry_ref(&self) -> &geo::GeometryCollection {
        &self.geometry
    }
}

// Locations we'll have KML / KMZ files for
// we require everything because we have to match our OSRM bounds
#[derive(Debug)]
pub enum Location {
    Uk,
    Ny,
    Ca,
}
