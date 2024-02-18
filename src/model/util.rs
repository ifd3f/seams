pub mod permissive_vec {
    use serde::{Deserialize, Deserializer, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum VecOrSingle<T> {
        Single(T),
        Multiple(Vec<T>),
    }

    pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Vec<T>, D::Error> {
        let intermediate: Option<VecOrSingle<T>> = Deserialize::deserialize(d)?;
        Ok(match intermediate {
            None => {
                vec![]
            }
            Some(VecOrSingle::Single(x)) => {
                vec![x]
            }
            Some(VecOrSingle::Multiple(xs)) => xs,
        })
    }
}
