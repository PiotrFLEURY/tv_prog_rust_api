use crate::data::models::XmlTv;
use dotenv::var;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub fn base_url() -> String {
    let base_url =
        var("XMLTV_BASE_URL").expect("XMLTV_BASE_URL must be set in the environment variables");
    if base_url.ends_with("/") {
        return base_url;
    }
    return base_url + "/";
}

pub fn xmltv_url_all() -> String {
    format!("{}xmltv.zip", base_url())
}

pub fn xmltv_url_tnt() -> String {
    format!("{}xmltv_tnt.zip", base_url())
}

pub fn xmltv_url_fr() -> String {
    format!("{}xmltv_fr.zip", base_url())
}

pub async fn fetch_xmltv_all() -> Result<XmlTv, String> {
    fetch_xmltv(xmltv_url_all()).await
}

pub async fn fetch_xmltv_tnt() -> Result<XmlTv, String> {
    fetch_xmltv(xmltv_url_tnt()).await
}

pub async fn fetch_xmltv_fr() -> Result<XmlTv, String> {
    fetch_xmltv(xmltv_url_fr()).await
}

pub async fn fetch_xmltv(request_url: String) -> Result<XmlTv, String> {
    println!("Fetching XML TV from {}", request_url);
    let response = reqwest::get(request_url).await.map_err(|e| e.to_string())?;

    let status = response.status();
    println!("Response status: {}", status);
    if !status.is_success() {
        eprintln!("Received error response: {}", status);
        Err(format!("Failed to fetch XMLTV data: {}", status))
    } else {
        println!("Extracting XMLTV data from zip file...");
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        let zip_bytes = Cursor::new(bytes);
        let mut archive = match ZipArchive::new(zip_bytes) {
            Ok(arch) => arch,
            Err(e) => {
                eprintln!("Failed to read zip archive: {}", e);
                return Err(format!("Failed to read zip archive: {}", e));
            }
        };

        for i in 0..archive.len() {
            let mut file = match archive.by_index(i) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to access file in zip archive: {}", e);
                    return Err(format!("Failed to access file in zip archive: {}", e));
                }
            };
            if file.name().ends_with(".xml") {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to read XML file from zip archive: {}", e);
                        return Err(format!("Failed to read XML file from zip archive: {}", e));
                    }
                }
                println!("Parsing XML TV data...");
                let xml_tv: XmlTv = match serde_xml_rs::from_str(&content) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to parse XML TV data: {}", e);
                        return Err(format!("Failed to parse XML TV data: {}", e));
                    }
                };
                return Ok(xml_tv);
            }
        }

        Err("No XML file found in the archive.".to_string())
    }
}
