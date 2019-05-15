
#[derive(Debug)]
pub struct Department {
    name: String,
    employees: Vec<crate::user::User>,
}

impl Department {
    pub fn new(name: String) -> Department {
        Department {
            name,
            employees: vec![],
        }
    }
}
