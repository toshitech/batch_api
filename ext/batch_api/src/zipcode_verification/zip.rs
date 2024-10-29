use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive};

pub fn rb_compress_kml_to_kmz(
    kml_path: String,
    output_kmz_path: String,
) -> Result<(), magnus::Error> {
    let (kml_path, output_kmz_path) = (Path::new(&kml_path), Path::new(&output_kmz_path));

    // Open the KML file to compress
    let kml_file = File::open(kml_path).map_err(|_| {
        magnus::Error::new(magnus::exception::io_error(), "No KML file at input path")
    })?;

    let kml_reader = BufReader::new(kml_file);

    // create the KMZ file (zip archive)
    let kmz_file = File::create(output_kmz_path).map_err(|_| {
        magnus::Error::new(magnus::exception::io_error(), "Failed to create output KMZ")
    })?;

    let mut zip = zip::ZipWriter::new(kmz_file);

    // No compression or use CompressionMethod::Deflated for compression
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file(
        Path::new(kml_path).file_name().unwrap().to_string_lossy(),
        options,
    )
    .map_err(|_| {
        magnus::Error::new(
            magnus::exception::io_error(),
            "Failed to create file in KMZ archive",
        )
    })?;

    // Copy the content from the KML file into the KMZ archive
    for byte in kml_reader.bytes() {
        let byte = byte.map_err(|_| {
            magnus::Error::new(
                magnus::exception::io_error(),
                "Failed to write bytes to KMZ",
            )
        })?;

        zip.write_all(&[byte]).map_err(|_| {
            magnus::Error::new(magnus::exception::io_error(), "No KML file at input path")
        })?;
    }

    // Finalize the KMZ file
    zip.finish().map_err(|_| {
        magnus::Error::new(
            magnus::exception::io_error(),
            "Failed to finish writing to KMZ",
        )
    })?;

    Ok(())
}

pub fn rb_uncompress_kmz_to_kml(
    kmz_path: String,
    output_kml_path: String,
) -> Result<(), magnus::Error> {
    let (kmz_path, output_kml_path) = (Path::new(&kmz_path), Path::new(&output_kml_path));

    let kmz_file = File::open(kmz_path).map_err(|_| {
        magnus::Error::new(magnus::exception::io_error(), "No KMZ file at input path")
    })?;

    // Create a zip archive from the file
    let mut archive = ZipArchive::new(kmz_file).map_err(|_| {
        magnus::Error::new(
            magnus::exception::io_error(),
            "Failed to read KMZ file at input path",
        )
    })?;

    // Iterate over the contents of the zip archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| {
            magnus::Error::new(
                magnus::exception::io_error(),
                "Failed to iterate through KMZ archive",
            )
        })?;

        // Look for the .kml file inside the .kmz
        if file.name().ends_with(".kml") {
            // Create a file to write the .kml content
            let mut output_file = File::create(output_kml_path).map_err(|_| {
                magnus::Error::new(
                    magnus::exception::io_error(),
                    "Failed to create output KML file",
                )
            })?;

            // Copy the content of the .kml file to the output
            std::io::copy(&mut file, &mut output_file).map_err(|_| {
                magnus::Error::new(
                    magnus::exception::io_error(),
                    "Failed to write to output KML file",
                )
            })?;

            return Ok(());
        }
    }
    Ok(())
}
