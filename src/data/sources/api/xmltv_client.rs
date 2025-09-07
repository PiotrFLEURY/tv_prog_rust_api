use crate::data::models::xmltv::XmlTv;
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

pub async fn fetch_xmltv_all() -> XmlTv {
    fetch_xmltv(xmltv_url_all()).await
}

pub async fn fetch_xmltv_tnt() -> XmlTv {
    fetch_xmltv(xmltv_url_tnt()).await
}

pub async fn fetch_xmltv_fr() -> XmlTv {
    fetch_xmltv(xmltv_url_fr()).await
}

pub async fn fetch_xmltv(request_url: String) -> XmlTv {
    println!("Fetching XML TV from {}", request_url);
    let response = reqwest::get(request_url).await.unwrap();

    let status = response.status();
    println!("Response status: {}", status);
    if !status.is_success() {
        eprintln!("Received error response: {}", status);
        panic!("Failed to fetch XMLTV data: {}", status);
    } else {
        println!("Extracting XMLTV data from zip file...");
        let bytes = response.bytes().await.unwrap();
        let zip_bytes = Cursor::new(bytes);
        let mut archive = ZipArchive::new(zip_bytes).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            if file.name().ends_with(".xml") {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                println!("Parsing XML TV data...");
                let xml_tv: XmlTv = serde_xml_rs::from_str(&content).unwrap();
                return xml_tv;
            }
        }

        panic!(
            "XMLTV data fetched successfully, but this is a placeholder function. Please implement the actual parsing logic."
        );
    }
}
