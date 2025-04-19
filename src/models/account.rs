pub struct Account {
    fname: String,
    lname: String,
}

impl Account {
    async fn new(fname: &str, lname: &str) -> Self {
		
        Self {
			fname: fname.into(),
			lname: lname.into()
		}
    }
}
