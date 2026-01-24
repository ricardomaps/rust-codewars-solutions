struct Omnibool {}

impl PartialEq<bool> for Omnibool {
    fn eq(&self, other: &bool) -> bool {
        true
    }
}

const omnibool: Omnibool = Omnibool {}; //perform you magic
