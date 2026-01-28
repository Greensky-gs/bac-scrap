use crate::result::Resultat;

#[derive(serde::Deserialize)]
struct TextResponse {
    results: Vec<Resultat>
}

pub fn get_url_for(name: &String) -> String {
    return format!("https://resultats.examens-concours.gouv.fr/api/BGT/publication?filtre={}&contexte=QkdULEExNiwyMDI1OkE6QkdULTIuMywxLCwsLA==", name);
}

pub fn fetch(url: &String) -> Option<Vec<Resultat>> {
    let response = reqwest::blocking::get(url);

    if let Err(e) = response {
        println!("Error occured: {:?}", e);
        return None;
    }
    let response = response.unwrap().text();
    if let Err(e) = response {
        println!("Error occured while parsing text: {:?}", e);
        return None;
    }
    let response = response.unwrap();

    let results: Result<TextResponse, serde_json::Error> = serde_json::from_str(&response);
    match results {
        Err(e) => {
            println!("Got an error while parsing to json : {:?}", e);
            return None
        },
        Ok(value) => {
            return Some(value.results);
        }
    }
}

pub fn wait(ms: u16) {
    let mut child = std::process::Command::new("sleep").arg(format!("{}", ms as f32 / 1000 as f32)).spawn().unwrap();
    let _ = child.wait().unwrap();
}
