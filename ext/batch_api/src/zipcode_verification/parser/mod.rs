use crate::zipcode_verification::zipcode::ZipcodeSector;
use kml::{Kml, KmlReader};

pub mod uk;
pub mod us;

pub trait Parse {
    fn read_kmz_file(path: &std::path::Path) -> Result<Kml, String> {
        let mut kmz_reader = KmlReader::<_, f64>::from_kmz_path(path)
            .map_err(|_| "Invalid KMZ file at path".to_string())?;
        // Try to read the KML data
        let kml_data = kmz_reader
            .read()
            .map_err(|_| "Unable to parse KMZ file at path".to_string())?;

        Ok(kml_data)
    }

    fn parse(path: &std::path::Path) -> Result<Vec<ZipcodeSector>, String>;
}
