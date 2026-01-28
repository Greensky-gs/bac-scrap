#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Resultat {
    pub nom: String,
    pub prenoms: String,
    pub resultat: String,
    pub homonyme: bool
}
