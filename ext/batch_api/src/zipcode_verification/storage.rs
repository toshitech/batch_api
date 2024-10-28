use geo::{Point, Within};
use std::path::Path;

use super::parser::Parse;
use super::parser::{uk, us};

use super::zipcode::ZipcodeSector;

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

    // Functions for our ruby interface
    pub fn rb_query(
        &self,
        location: String,
        lat: f64,
        lng: f64,
    ) -> Result<Option<String>, magnus::Error> {
        let point = Point::new(lng, lat);
        let val = self.0.borrow().query_zipcode_sector(&location, point);

        Ok(val)
    }

    // loading methods
    pub fn rb_load_uk_sectors_from_kmz_file(&self, path: String) -> Result<(), magnus::Error> {
        let path = Path::new(&path);

        self.0
            .borrow_mut()
            .load_uk_sectors_from_kmz_file(path)
            .map_err(|err| magnus::Error::new(magnus::exception::arg_error(), err))
    }

    pub fn rb_load_ny_sectors_from_kmz_file(&self, path: String) -> Result<(), magnus::Error> {
        let path = Path::new(&path);

        self.0
            .borrow_mut()
            .load_ny_sectors_from_kmz_file(path)
            .map_err(|err| magnus::Error::new(magnus::exception::arg_error(), err))
    }

    pub fn rb_load_nj_sectors_from_kmz_file(&self, path: String) -> Result<(), magnus::Error> {
        let path = Path::new(&path);

        self.0
            .borrow_mut()
            .load_nj_sectors_from_kmz_file(path)
            .map_err(|err| magnus::Error::new(magnus::exception::arg_error(), err))
    }

    pub fn rb_load_ca_sectors_from_kmz_file(&self, path: String) -> Result<(), magnus::Error> {
        let path = Path::new(&path);

        self.0
            .borrow_mut()
            .load_ca_sectors_from_kmz_file(path)
            .map_err(|err| magnus::Error::new(magnus::exception::arg_error(), err))
    }
}

// containts an initialisable struct we can hold in memory
// that loads and stores the zipcode sectors for each location
// Default derive will set the option fields to None
#[derive(Default)]
struct MemStore {
    // we're expecting like ~80mb of uncompressed data
    uk_zipcode_sectors: Vec<ZipcodeSector>,
    ny_zipcode_sectors: Vec<ZipcodeSector>,
    nj_zipcode_sectors: Vec<ZipcodeSector>,
    ca_zipcode_sectors: Vec<ZipcodeSector>,
}

impl MemStore {
    // called by ruby
    pub fn query_zipcode_sector(&self, location: &str, point: Point) -> Option<String> {
        let sectors = match location {
            "uk" => &self.uk_zipcode_sectors,
            "ny" => &self.ny_zipcode_sectors,
            "nj" => &self.nj_zipcode_sectors,
            "ca" => &self.ca_zipcode_sectors,
            _ => return None,
        };

        if !sectors.is_empty() {
            for sect in sectors.iter() {
                if point.is_within(sect.geometry_ref()) {
                    return Some(sect.name());
                }
            }
        }

        None
    }

    // Entire of UK
    pub fn load_uk_sectors_from_kmz_file(&mut self, path: &Path) -> Result<(), String> {
        let mut zipcode_sectors = uk::Parser::parse(path)?;
        self.uk_zipcode_sectors.append(&mut zipcode_sectors);
        Ok(())
    }

    // US state
    pub fn load_ny_sectors_from_kmz_file(&mut self, path: &Path) -> Result<(), String> {
        let mut zipcode_sectors = us::Parser::parse(path)?;
        self.ny_zipcode_sectors.append(&mut zipcode_sectors);
        Ok(())
    }

    pub fn load_nj_sectors_from_kmz_file(&mut self, path: &Path) -> Result<(), String> {
        let mut zipcode_sectors = us::Parser::parse(path)?;
        self.nj_zipcode_sectors.append(&mut zipcode_sectors);
        Ok(())
    }

    pub fn load_ca_sectors_from_kmz_file(&mut self, path: &Path) -> Result<(), String> {
        let mut zipcode_sectors = us::Parser::parse(path)?;
        self.ca_zipcode_sectors.append(&mut zipcode_sectors);
        Ok(())
    }
}
