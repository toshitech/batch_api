// CA, NY, NJ and other US KML files are in the same format

use crate::zipcode_verification::parser::Parse;
use crate::zipcode_verification::zipcode::ZipcodeSector;

use kml::{types::Placemark, Kml};

pub struct Parser;

impl Parse for Parser {
    fn parse(path: &std::path::Path) -> Result<Vec<ZipcodeSector>, String> {
        let kml_data = Self::read_kmz_file(path)?;

        // different format of document so these are different
        // we can just iterate over the elements in this one instead
        // of messing around with recursion for different nesting

        // Ensure the KML document contains the root <Document>

        let kml_base_element = if let Kml::KmlDocument(kml_base_element) = kml_data {
            kml_base_element
        } else {
            return Err("KML file does not contain a <Document> element".into());
        };

        let mut zip_codes = Vec::new();

        // iterate through elements in the document
        if let Some(Kml::Document { attrs: _, elements }) = kml_base_element.elements.first() {
            for element in elements {
                if let Kml::Placemark(placemark) = element {
                    let name = extract_name(placemark)?;
                    let geometry: geo::GeometryCollection<f64> =
                        kml::quick_collection(element.to_owned()).unwrap();
                    zip_codes.push(ZipcodeSector::new(name, geometry));
                }
            }
        } else {
            return Err("Invalid KML document format".to_string());
        }

        Ok(zip_codes)
    }
}

// pull the name out of a placemark
fn extract_name(placemark: &Placemark) -> Result<String, String> {
    let ele = placemark.children.first();
    if let Some(extended_data) = ele {
        let ele = extended_data.children.first();
        if let Some(data_element) = ele {
            let ele = data_element.children.first();
            if let Some(value_element) = ele {
                if let Some(ref value_element_content) = value_element.content {
                    return Ok(value_element_content.to_owned());
                }
            }
        }
    }

    Err("Unable to get name from placemark".to_string())
}
