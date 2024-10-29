#[derive(Debug)]
pub struct ZipcodeSector {
    name: String,
    geometries: geo::GeometryCollection,
}

impl ZipcodeSector {
    pub fn new(name: String, geometries: geo::GeometryCollection) -> Self {
        ZipcodeSector { name, geometries }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn geometry_ref(&self) -> &geo::GeometryCollection {
        &self.geometries
    }
}
