use rustbasic_core::model;

model! {
    table: "users",
    fillable: [name, email, password],
    Model {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub email_verified_at: Option<String>,
        pub password: String,
        pub remember_token: Option<String>,
    }
}

impl Model {

    /// Accessor: Get formatted name and email representation
    pub fn name_and_email(&self) -> String {
        format!("{} ({})", self.name, self.email)
    }

    /// Mutator: Hash and set the user's password securely
    pub fn set_password(&mut self, plain: &str) {
        self.password = rustbasic_core::bcrypt::hash(plain, rustbasic_core::bcrypt::DEFAULT_COST).unwrap();
    }

    /// API Resource: Transform model into safe public JSON response (hiding password, etc.)
    pub fn to_resource(&self) -> rustbasic_core::serde_json::Value {
        rustbasic_core::serde_json::json!({
            "id": self.id,
            "name": self.name,
            "email": self.email,
            "email_verified": self.email_verified_at.is_some(),
        })
    }
}
