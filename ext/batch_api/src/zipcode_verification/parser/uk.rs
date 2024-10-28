// parse the KML file for UK which contains a different structure
// to the US ones.
// https://www.doogal.co.uk/kml/ZipcodeSectors.kml

use crate::zipcode_verification::parser::Parse;
use crate::zipcode_verification::zipcode::ZipcodeSector;

use kml::Kml;

pub struct Parser;

impl Parse for Parser {
    fn parse(path: &std::path::Path) -> Result<Vec<ZipcodeSector>, String> {
        let kml_data = Self::read_kmz_file(path)?;
        // Check if the KML data is a valid KML document
        if let Kml::KmlDocument(kml_base_element) = kml_data {
            if let Some(Kml::Document { attrs: _, elements }) = kml_base_element.elements.first() {
                let mut postcode_sectors: Vec<ZipcodeSector> = Vec::new();

                // Parse and add the postcode sectors
                add_zipcode_sectors_from_folders(elements, &mut postcode_sectors);

                Ok(postcode_sectors)
            } else {
                Err("Invalid KML document format".to_string())
            }
        } else {
            Err("Not a valid KML document".to_string())
        }
    }
}

// because the UK file has some different nesting of folders
// we have to recursively search the folder tree to return placemarks
// Might change how we parse these to avoid in future, will see how
// I end up tackling the other KMZ files first
pub fn add_zipcode_sectors_from_folders(
    folder_elements: &[Kml],
    postcode_sectors_buff: &mut Vec<ZipcodeSector>,
) {
    for element in folder_elements.iter() {
        match element {
            Kml::Folder { attrs: _, elements } => {
                add_zipcode_sectors_from_folders(elements, postcode_sectors_buff)
            }
            Kml::Placemark(placemark) => {
                if placemark.name.is_some() && placemark.geometry.is_some() {
                    // we pull it out as a placemark to select the name
                    let name = placemark.name.to_owned().unwrap();
                    // convert the element to a geo type so we can use the useful algorithms
                    let geom_coll: geo::GeometryCollection<f64> =
                        kml::quick_collection(element.to_owned()).unwrap();
                    let sector = ZipcodeSector::new(name.to_owned(), geom_coll);
                    postcode_sectors_buff.push(sector);
                }
            }
            _ => {}
        }
    }
}
