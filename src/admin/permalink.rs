use regex::Regex;
use std::str::FromStr;

enum PermalinkStructure {
    Plain,
    PostName
}
impl FromStr for PermalinkStructure {

    type Err = ();

    fn from_str(input: &str) -> Result<PermalinkStructure, Self::Err> {
        match input {
            "Plain"  => Ok(PermalinkStructure::Plain),
            "PostName"  => Ok(PermalinkStructure::PostName),
            _      => Ok(PermalinkStructure::PostName),
        }
    }
}
pub fn extract_params(url:&str)-> &str {
    let re = Regex::new(r"\/\w+?\?p=(.*)").unwrap(); // default plain url
    // (?:\w+:)?\/\/[^\/]+\/([^?#]+) // post name url
    re.is_match(url);
    url
}

pub fn extract_url(url: &str, permalink_type: &str)-> (bool, Vec<String>){
    let mut url_params = Vec::new();
    let as_permalink_structure: PermalinkStructure = PermalinkStructure::from_str(permalink_type).unwrap();
    let regex = match as_permalink_structure {
        PermalinkStructure::Plain => r"\?p=(.*)",
        PermalinkStructure::PostName => r"/([a-zA-Z\-]+)"
    };
    let re = Regex::new(regex).unwrap();
    let is_match = re.is_match(url);
    if !is_match {
        return (false, url_params)
    }
    for cap in re.captures_iter(url) {
        println!("{:?}", cap);
        url_params.push(cap[1].to_string());
    }
    return (false, url_params)
}