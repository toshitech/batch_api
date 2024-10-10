use geo::{Point, Within};
use kml::{Kml, KmlReader};
use std::path::Path;

use super::zipcode::{Location, ZipcodeSector};
use std::cell::RefCell;

// we have to use refcell + newtype pattern to mutate our struct
// since magnus doesnt allow us to have mutable references to self
// with struct methods (&mut self) exposed to ruby interface.
// So we'll just use this as our Ruby wrapper struct
#[magnus::wrap(class = "BatchApi::ZipcodeVerification::MemStore", free_immediately)]
pub struct MutMemStore(RefCell<MemStore>);

impl MutMemStore {
    pub fn rb_new() -> MutMemStore {
        MutMemStore(RefCell::new(MemStore::default()))
    }

    pub fn rb_load_uk_sectors_from_kmz_file(&self, path: String) -> Result<(), magnus::Error> {
        let path = Path::new(&path);

        self.0
            .borrow_mut()
            .load_uk_sectors_from_kmz_file(path)
            .map_err(|err| magnus::Error::new(magnus::exception::arg_error(), err))
    }

    pub fn rb_query_uk_zipcode_sector(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<Option<String>, magnus::Error> {
        let point = Point::new(lng, lat);
        let val = self.0.borrow().query_zipcode_sector(Location::Uk, point);

        Ok(val)
    }
}

// containts an initialisable struct we can hold in memory
// that loads and stores the zipcode sectors for each location
// Default derive will set the option fields to None
#[derive(Default)]
struct MemStore {
    // full country
    // we're expecting like 40mb of uncompressed data in memory for this
    uk_zipcode_sectors: Option<Vec<ZipcodeSector>>,
    // full state
    _ny_zipcode_sectors: Option<Vec<ZipcodeSector>>,
    // just the city
    _la_zipcode_sectors: Option<Vec<ZipcodeSector>>,
}

impl MemStore {
    // called by ruby
    pub fn query_zipcode_sector(&self, location: Location, point: Point) -> Option<String> {
        let sectors = match location {
            Location::Uk => &self.uk_zipcode_sectors,
            Location::Ny => {
                todo!("not implemented yet")
            }
            Location::Ca => {
                todo!("not implemented yet")
            }
        };

        if let Some(sectors) = sectors {
            for sect in sectors.iter() {
                if point.is_within(sect.geometry_ref()) {
                    return Some(sect.name());
                }
            }
        }

        None
    }
    // All locations load their data differently due to
    // different formats of the underlying KML data
    pub fn _load_sectors_from_ny_kmz_file(&mut self, _path: &Path) {
        todo!["Need to get a NY KMZ file"];
    }

    pub fn _load_sectors_from_la_kmz_file(&mut self, _path: &Path) {
        todo!["Need to get an LA KMZ file"];
    }

    // You can find a KML file here https://www.doogal.co.uk/kml/ZipcodeSectors.kml
    fn load_uk_sectors_from_kmz_file(&mut self, path: &std::path::Path) -> Result<(), String> {
        // Try to create a KML reader from the KMZ file path
        let mut kmz_reader = KmlReader::<_, f64>::from_kmz_path(path)
            .map_err(|_| "Invalid KMZ file at path".to_string())?;
        // Try to read the KML data
        let kml_data = kmz_reader
            .read()
            .map_err(|_| "Unable to parse KMZ file at path".to_string())?;

        // Check if the KML data is a valid KML document
        if let Kml::KmlDocument(kml_base_element) = kml_data {
            if let Some(Kml::Document { attrs: _, elements }) = kml_base_element.elements.first() {
                let mut postcode_sectors: Vec<ZipcodeSector> = Vec::new();

                // Parse and add the postcode sectors
                add_zipcode_sectors_from_folders(elements, &mut postcode_sectors);

                self.uk_zipcode_sectors = Some(postcode_sectors);
            } else {
                return Err("Invalid KML document format".to_string());
            }
        } else {
            return Err("Not a valid KML document".to_string());
        }

        Ok(())
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
                    let sector = ZipcodeSector::new(name.to_owned(), geom_coll, Location::Uk);
                    postcode_sectors_buff.push(sector);
                }
            }
            _ => {}
        }
    }
}
